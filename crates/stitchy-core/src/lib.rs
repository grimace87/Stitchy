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
//! // Select image files in current directory, take first 3 by alphabetic order on file name
//! use stitchy_core::{ImageFiles, OrderBy, TakeFrom};
//! let image_files = ImageFiles::builder()
//!     .add_current_directory(vec!["..", "..", "images", "demo"]).unwrap()
//!     .build().unwrap()
//!     .sort_and_truncate_by(3, OrderBy::Alphabetic, TakeFrom::Start, false).unwrap();
//!
//! // Stitch images in a horizontal line, restricting the width to 1000 pixels
//! use stitchy_core::{Stitch, AlignmentMode};
//! let stitch = Stitch::builder()
//!     .image_files(image_files).unwrap()
//!     .width_limit(1000)
//!     .alignment(AlignmentMode::Horizontal)
//!     .stitch();
//!
//! assert!(stitch.is_ok());
//! ```

mod enums;
mod files;
mod stitch;

#[cfg(test)]
mod tests;

/// Enums used for configuring the image stitching process
pub use enums::{OrderBy, TakeFrom};

/// Types used for loading files and passing them into the image stitching process
pub use files::{
    builder::ImageFilesBuilder,
    image_types::ImageFiles,
    path::{FilePath, FilePathWithMetadata},
    raw::{RawBufferLocation, RawBufferProperties},
    util::extension_formats,
    FileLocation, FileProperties,
};

#[cfg(unix)]
pub use files::fd::{OwnedRawFdLocation, OwnedRawFdProperties};

/// Type used for running the image stitching process
pub use stitch::{builder::StitchBuilder, AlignmentMode, Stitch};

/// File utilities, used by the CLI crate
pub mod util {
    pub use crate::files::util::make_size_string;
}

/// Re-exports from the [image](https://crates.io/crates/image) crate
pub mod image {
    pub use image::{
        codecs::{
            gif::GifEncoder,
            jpeg::JpegEncoder,
            png::{CompressionType as PngCompressionType, FilterType as PngFilterType, PngEncoder}
        },
        imageops::FilterType,
        metadata::Orientation,
        DynamicImage, Frame, GenericImage, ImageDecoder,
        ImageError, ImageFormat,
    };
}
