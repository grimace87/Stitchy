
use crate::{AlignmentMode, Stitch, ImageFiles, image::DynamicImage, FileProperties};
use std::fmt::Debug;

/// Builder for a Stitch.
///
/// Performs all of the configuration for the stitch, and triggers generation of the output image.
/// Source images are provided using the [ImageFiles] struct, which has its own builder.
///
/// See crate-level documentation for examples.
#[derive(Default, Debug)]
pub struct StitchBuilder {
    pub(crate) images: Vec<DynamicImage>,
    pub(crate) alignment: AlignmentMode,
    pub(crate) width_limit: u32,
    pub(crate) height_limit: u32
}

impl StitchBuilder {

    pub fn images(self, images: Vec<DynamicImage>) -> StitchBuilder {
        StitchBuilder {
            images,
            ..self
        }
    }

    pub fn image_files<P>(self, files: ImageFiles<P>) -> Result<StitchBuilder, String>
        where P: FileProperties + Debug
    {
        let images = files.into_image_contents(false)?;
        Ok(StitchBuilder {
            images,
            ..self
        })
    }

    pub fn alignment(self, alignment: AlignmentMode) -> StitchBuilder {
        StitchBuilder {
            alignment,
            ..self
        }
    }

    pub fn width_limit(self, width_limit: u32) -> StitchBuilder {
        StitchBuilder {
            width_limit,
            ..self
        }
    }

    pub fn height_limit(self, height_limit: u32) -> StitchBuilder {
        StitchBuilder {
            height_limit,
            ..self
        }
    }

    pub fn stitch(self) -> Result<DynamicImage, String> {
        if self.images.is_empty() {
            return Err("No images to stitch".to_owned())
        }
        Stitch::new(self.images, self.alignment, self.width_limit, self.height_limit)
            .stitch()
    }
}
