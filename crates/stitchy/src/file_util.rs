
use crate::Opt;
use stitchy_core::{ImageFormat, ImageFiles};
use image::{DynamicImage, ImageOutputFormat};
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn next_available_output(sources: &ImageFiles, options: &Opt) -> Result<PathBuf, String> {

    let target_extension = determine_output_format(sources, options)?
        .get_main_extension();

    // Get current path, check if the default file name exists, if not return it
    let mut current_path: PathBuf = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Could not access current directory"))
    };
    let mut un_numbered_file_exists = false;
    for &extension in ImageFormat::allowed_extensions().iter() {
        current_path.push(format!("stitch.{}", extension));
        if current_path.is_file() {
            un_numbered_file_exists = true;
            current_path.pop();
            break;
        }
        current_path.pop();
    }
    if !un_numbered_file_exists {
        let mut path = current_path.clone();
        path.push(format!("stitch.{}", target_extension));
        return Ok(path);
    }

    // Check file names until a usable one is found
    let mut i = 1usize;
    while i < 1000 {
        let mut numbered_file_exists = false;
        for &extension in ImageFormat::allowed_extensions().iter() {
            let file_name: String = format!("stitch_{}.{}", i, extension);
            current_path.push(file_name);
            if current_path.is_file() {
                numbered_file_exists = true;
            }
            current_path.pop();
            if numbered_file_exists {
                break;
            }
        }
        if !numbered_file_exists {
            let mut path = current_path.clone();
            path.push(format!("stitch_{}.{}", i, target_extension));
            return Ok(path);
        }
        i += 1;
    };
    Err(String::from("Did not find a usable file name - if you have 1000 stitches, please move or delete some."))
}

pub fn write_image_to_file(image: DynamicImage, file_path: &Path, format: ImageFormat, quality: usize) -> Result<(), String> {
    let mut file_writer = File::create(file_path).unwrap();
    let format = match format {
        ImageFormat::Jpeg => ImageOutputFormat::Jpeg(quality as u8),
        ImageFormat::Png => ImageOutputFormat::Png,
        ImageFormat::Gif => ImageOutputFormat::Gif,
        ImageFormat::Bmp => ImageOutputFormat::Bmp,
        ImageFormat::Unspecified => ImageOutputFormat::Jpeg(100u8) // Should not reach this point
    };
    match image.write_to(&mut file_writer, format) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("Failed to generate output file - {}", error))
    }
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

pub fn determine_output_format(sources: &ImageFiles, options: &Opt) -> Result<ImageFormat, String> {

    let mut image_format: ImageFormat = options.get_requested_image_format();

    // Check if no format was specified, but all source images are the same
    if image_format == ImageFormat::Unspecified {
        image_format = sources.common_format_in_sources();
        if image_format == ImageFormat::Unspecified {
            image_format = ImageFormat::Jpeg;
        }
        if image_format != ImageFormat::Jpeg && options.quality != 100 {
            return Err(format!(
                "Output file with extension .{} cannot use a quality setting.",
                image_format.get_main_extension()));
        }
    }

    Ok(image_format)
}
