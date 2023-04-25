
use crate::{AlignmentMode, Stitch, ImageFiles, image::DynamicImage};

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
