mod enums;
mod files;
mod stitch;

#[cfg(test)]
mod tests;

pub use enums::{ImageFormat, OrderBy, TakeFrom};
pub use files::{image::ImageFiles, builder::ImageFilesBuilder};
pub use stitch::{Stitch, AlignmentMode, Axis, builder::StitchBuilder};

pub mod util {
    pub use crate::files::util::make_size_string;
}

pub mod image {
    pub use image::{DynamicImage, ImageOutputFormat, GenericImage};
}
