
use crate::Opt;
use stitchy_core::{ImageFiles, FilePathWithMetadata, image::{GifEncoder, Frame, ImageFormat, DynamicImage, JpegEncoder, PngCompressionType, PngEncoder, PngFilterType}};
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

pub fn to_absolute_dir(path_string: &String) -> Result<PathBuf, String> {
    let path = PathBuf::from(path_string)
        .canonicalize()
        .map_err(|e| format!("Cannot read path: {}", e))?;
    if !path.is_dir() {
        return Err(format!("Not a directory: {}", path_string));
    }
    Ok(path)
}

pub fn next_available_output(
    sources: &ImageFiles<FilePathWithMetadata>,
    options: &Opt
) -> Result<PathBuf, String> {

    let target_extension = ImageFiles::<FilePathWithMetadata>
        ::get_main_extension(determine_output_format(sources, options)?)
        .unwrap_or("jpg");

    let mut output_file_path: PathBuf = match &options.output_dir {
        Some(output_path) => {
            to_absolute_dir(output_path)?
        },
        None => {
            match std::env::current_dir() {
                Ok(dir) => dir,
                Err(_) => return Err(String::from("Could not access current directory"))
            }
        }
    };

    // Get current path, check if the default file name exists, if not return it
    let mut un_numbered_file_exists = false;
    for &extension in ImageFiles::<FilePathWithMetadata>::allowed_extensions().iter() {
        output_file_path.push(format!("stitch.{}", extension));
        if output_file_path.is_file() {
            un_numbered_file_exists = true;
            output_file_path.pop();
            break;
        }
        output_file_path.pop();
    }
    if !un_numbered_file_exists {
        let mut path = output_file_path.clone();
        path.push(format!("stitch.{}", target_extension));
        return Ok(path);
    }

    // Check file names until a usable one is found
    let mut i = 1usize;
    while i < 1000 {
        let mut numbered_file_exists = false;
        for &extension in ImageFiles::<FilePathWithMetadata>::allowed_extensions().iter() {
            let file_name: String = format!("stitch_{}.{}", i, extension);
            output_file_path.push(file_name);
            if output_file_path.is_file() {
                numbered_file_exists = true;
            }
            output_file_path.pop();
            if numbered_file_exists {
                break;
            }
        }
        if !numbered_file_exists {
            let mut path = output_file_path.clone();
            path.push(format!("stitch_{}.{}", i, target_extension));
            return Ok(path);
        }
        i += 1;
    };
    Err(String::from("Did not find a usable file name - if you have 1000 stitches, please move or delete some."))
}

pub fn write_image_to_file(
    image: DynamicImage,
    file_path: &Path,
    format: Option<ImageFormat>,
    quality: usize,
    encode_smallest: bool
) -> Result<(), String> {
    let mut file_writer = BufWriter::new(File::create(file_path).unwrap());
    let result = match format {
        None => JpegEncoder::new_with_quality(file_writer, 100).encode_image(&image),
        Some(ImageFormat::Jpeg) => { JpegEncoder::new_with_quality(file_writer, quality as u8).encode_image(&image) },
        Some(ImageFormat::Png) => {
            let mode = match encode_smallest {
                true => PngCompressionType::Best,
                false => PngCompressionType::Fast
            };
            let encoder = PngEncoder::new_with_quality(file_writer, mode, PngFilterType::default());
            image.write_with_encoder(encoder)
        },
        Some(ImageFormat::Gif) => {
            let speed = match encode_smallest {
                true => 1,
                false => 10
            };
            let mut encoder = GifEncoder::new_with_speed(file_writer, speed);
            encoder.encode_frame(Frame::new(image.to_rgba8()))
        },
        Some(ImageFormat::Bmp) => image.write_to(&mut file_writer, ImageFormat::Bmp),
        Some(ImageFormat::WebP) => image.write_to(&mut file_writer, ImageFormat::WebP),
        Some(other_format) => { panic!("Internal error: found format {:?}", other_format) },
    };
    result.map_err(|e| format!("Failed to generate output file - {}", e))
}

pub fn size_of_file(file_path: &Path) -> Result<u64, String> {
    let length_bytes = file_path.metadata()
        .map_err(|_| "File metadata could not be read.".to_owned())?
        .len();
    Ok(length_bytes)
}

pub fn make_ratio_string(input_size: u64, output_size: u64) -> String {
    if input_size == 0 {
        return "-".to_owned();
    }
    let ratio = (output_size as f64) / (input_size as f64);
    format!("{:.0}%", ratio * 100.0)
}

pub fn determine_output_format(
    sources: &ImageFiles<FilePathWithMetadata>,
    options: &Opt
) -> Result<ImageFormat, String> {

    let requested_format: Option<ImageFormat> = options.get_requested_image_format();

    // Check if no format was specified, but all source images are the same
    let image_format = match requested_format {
        None => {
            let common_format = sources.common_format_in_sources();
            match common_format {
                None => ImageFormat::Jpeg,
                Some(format) => {
                    if format != ImageFormat::Jpeg && options.quality != 100 {
                        return Err(format!(
                            "Output format {:?} cannot use a quality setting.",
                            format));
                    }
                    format
                }
            }
        },
        Some(format) => format
    };

    Ok(image_format)
}
