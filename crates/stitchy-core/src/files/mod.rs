pub mod builder;

#[cfg(unix)]
pub mod fd;

pub mod image_types;
pub mod path;
pub mod util;

use image::{
    codecs::{jpeg::JpegDecoder, webp::WebPDecoder},
    metadata::Orientation,
    DynamicImage, ImageDecoder, ImageError, ImageFormat,
};
use std::fs::File;
use std::io::BufReader;
use std::time::SystemTime;

/// Representation of a file's location.
pub trait FileLocation<P: FileProperties> {
    fn is_file(&self) -> Result<bool, String>;
    fn extension(&self) -> Result<String, String>;
    fn into_properties(self) -> Result<P, String>;
}

/// Functions to get useful properties of files.
/// Implementers of this trait are representations of files. The files are not necessarily required to be open, or to
/// have been opened or read.
pub trait FileProperties {
    fn infer_format(&self) -> Option<ImageFormat>;
    fn into_image_contents(self, print_info: bool) -> Result<DynamicImage, String>;
    fn file_size(&self) -> u64;
    fn modify_time(&self) -> SystemTime;
    fn full_path(&self) -> Option<&String>;
    fn orientation(&self) -> Result<Orientation, String>;

    fn decode_orientation(&self, file: &File) -> Result<Orientation, String> {
        let format = match self.infer_format() {
            Some(format) => format,
            None => {
                return Ok(Orientation::NoTransforms);
            }
        };

        let full_path_label = match self.full_path() {
            Some(string) => string.as_str(),
            None => "(path unknown)",
        };
        let reader = BufReader::new(file);

        match format {
            ImageFormat::Jpeg => {
                Self::decode_orientation_from_codec(full_path_label, JpegDecoder::new(reader))
            }
            ImageFormat::WebP => {
                Self::decode_orientation_from_codec(full_path_label, WebPDecoder::new(reader))
            }
            _ => Ok(Orientation::NoTransforms),
        }
    }

    fn decode_orientation_from_codec<T: ImageDecoder>(
        full_path_label: &str,
        decoder_result: Result<T, ImageError>,
    ) -> Result<Orientation, String> {
        let mut decoder =
            decoder_result.map_err(|e| format!("Error decoding {}: {:?}", full_path_label, e))?;
        let orientation = decoder
            .orientation()
            .map_err(|e| format!("Cannot decode metadata in {}: {:?}", full_path_label, e))?;
        Ok(orientation)
    }
}
