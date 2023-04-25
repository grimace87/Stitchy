//! Joining images together into a single image.
//!
//! Provides two processes, each configured using the builder pattern:
//! - Selecting image files
//! - Copying the (typically resized) contents of those files into a single output image
//!
//! The first process is performed by the [`ImageFiles`] struct and its associated builder pattern.
//! Files are added either individually or from a directory.
//!
//! The second process is performed by the [`Stitch`] struct and its associated builder pattern.
//! The configuration sets the output image size and layout of the output image. The output image
//! is returned as an in-memory struct of type [`DynamicImage`], re-exported from the image crate
//! (see the [image crate on crates.io](https://crates.io/crates/image)).
//!
//! # Examples
//!
//! ```
//! // Select image files in current directory
//! use stitchy_core::ImageFiles;
//! let image_files = ImageFiles::builder()
//!     .add_current_directory(vec![]).unwrap()
//!     .build().unwrap();
//!
//! // Stitch images in a horizontal line, restricting the width to 1000 pixels
//! use stitchy_core::{Stitch, AlignmentMode};
//! let image = Stitch::builder()
//!     .image_files(image_files)
//!     .width_limit(1000)
//!     .alignment(AlignmentMode::Horizontal)
//!     .stitch().unwrap();
//! ```

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
