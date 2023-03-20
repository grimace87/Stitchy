
use crate::enums::ImageFormat;
use std::fs::File;
use std::path::Path;
use image::{DynamicImage, ImageOutputFormat};

const BYTES_KIB: u64 = 1024;
const BYTES_MIB: u64 = 1024 * 1024;

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

pub(crate) fn make_size_string(length_bytes: u64) -> String {
    match length_bytes {
        l if l < BYTES_KIB => format!(
            "{} bytes", l
        ),
        l if l < 10 * BYTES_KIB => format!(
            "{}.{} KiB", l / BYTES_KIB, (10 * (l % BYTES_KIB)) / BYTES_KIB
        ),
        l if l < BYTES_MIB => format!(
            "{} KiB", l / BYTES_KIB
        ),
        l if l < 10 * BYTES_MIB => format!(
            "{}.{} MiB", l / BYTES_MIB, (10 * (l % BYTES_MIB)) / BYTES_MIB
        ),
        l => format!("{} MiB", l / BYTES_MIB)
    }
}

pub(crate) fn make_ratio_string(input_size: u64, output_size: u64) -> String {
    if input_size == 0 {
        return "-".to_owned();
    }
    let ratio = (output_size as f64) / (input_size as f64);
    format!("{:.0}%", ratio * 100.0)
}
