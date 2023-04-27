
use crate::{AlignmentMode, Stitch, ImageFiles, image::DynamicImage};

/// Builder for a Stitch.
///
/// Performs all of the configuration for the stitch, and triggers generation of the output image.
/// Source images are provided using the [ImageFiles] struct, which has its own builder.
///
/// # Examples
///
/// ```
/// use stitchy_core::{Stitch, AlignmentMode};
/// let stitch = Stitch::builder()
///     .image_files(images).unwrap()
///     .alignment(AlignmentMode::Horizontal)
///     .width_limit(1000)
///     .stitch().unwrap();
/// ```
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

    pub fn image_files(self, files: ImageFiles) -> Result<StitchBuilder, String> {
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
        Stitch::new(self.images, self.alignment, self.width_limit, self.height_limit)
            .stitch()
    }
}
