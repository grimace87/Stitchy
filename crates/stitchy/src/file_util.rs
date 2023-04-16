
use stitchy_core::ImageFormat;
use image::{DynamicImage, ImageOutputFormat};
use std::fs::File;
use std::path::Path;

pub(crate) fn write_image_to_file(image: DynamicImage, file_path: &Path, format: ImageFormat, quality: usize) -> Result<(), String> {
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

pub(crate) fn size_of_file(file_path: &Path) -> Result<u64, String> {
    let length_bytes = file_path.metadata()
        .map_err(|_| "File metadata could not be read.".to_owned())?
        .len();
    Ok(length_bytes)
}

pub(crate) fn make_ratio_string(input_size: u64, output_size: u64) -> String {
    if input_size == 0 {
        return "-".to_owned();
    }
    let ratio = (output_size as f64) / (input_size as f64);
    format!("{:.0}%", ratio * 100.0)
}
