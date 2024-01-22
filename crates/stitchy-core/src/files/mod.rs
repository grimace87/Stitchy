
pub mod builder;
pub mod image;
pub mod path;
pub mod util;

use crate::image::{DynamicImage, ImageFormat};
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
}
