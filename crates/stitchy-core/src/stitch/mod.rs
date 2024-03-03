
pub mod builder;
pub mod pen;

#[cfg(test)]
mod tests;

use crate::{
    StitchBuilder,
    image::{DynamicImage, GenericImage},
    stitch::pen::{ImageRect, ImageGridPen, HorizontalGridPen, VerticalGridPen}
};

/// Layout configuration for the stitched result.
///
/// Options include:
/// - Grid, where a guess is made at a sensible number of rows and columns
/// - Horizontal, where all images are placed in a single row
/// - Vertical, where all images are placed in a single column
///
/// The grid layout is based on a guess of which should be the main axis (either horizontal or
/// vertical), then guessing the main axis and cross axis dimensions, and then stitching images
/// along the main axis until full and moving along the cross axis. Currently, this cannot be
/// controlled.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum AlignmentMode {
    #[default]
    Grid,
    Horizontal,
    Vertical
}

/// An approximate aspect ratio class
enum AspectType {
    Wide,
    Portrait,
    Squarish
}

impl AspectType {
    pub fn get_aspect_from_dims(w: u32, h: u32) -> AspectType {
        let aspect_ratio: f32 = (w as f32) / (h as f32);
        if aspect_ratio > 1.25f32 {
            AspectType::Wide
        } else if aspect_ratio < 0.8f32 {
            AspectType::Portrait
        } else {
            AspectType::Squarish
        }
    }
}

/// The full set of inputs for a stitch operation, including the source images and the layout
/// that the output will take. Use the [StitchBuilder] for the entire stitching process.
pub struct Stitch {
    images: Vec<DynamicImage>,
    axis_pen: Box<dyn ImageGridPen>,
    image_rects: Vec<ImageRect>
}

impl Stitch {

    pub fn builder() -> StitchBuilder {
        StitchBuilder::default()
    }

    pub(crate) fn new(
        images: Vec<DynamicImage>,
        alignment: AlignmentMode,
        width_limit: u32,
        height_limit: u32
    ) -> Stitch {

        let mut axis_pen = Self::make_axis_pen(alignment, &images);
        let unscaled_image_rects = axis_pen.generate_output_rects(&images);
        let image_rects = match width_limit > 0 || height_limit > 0 {
            true => axis_pen.scale_image_rects(unscaled_image_rects, width_limit, height_limit),
            false => unscaled_image_rects
        };

        Stitch {
            images,
            axis_pen,
            image_rects
        }
    }

    /// Creates a "pen" which draws images either horizontally or vertically as the primary axis.
    /// The pen draws in this direction until the images per line have been drawn, then moves to
    /// the next line.
    fn make_axis_pen(alignment: AlignmentMode, images: &Vec<DynamicImage>) -> Box<dyn ImageGridPen> {
        let image_count = images.len();

        // Check for very particular alignment modes
        if alignment == AlignmentMode::Horizontal {
            let smallest_height = Self::find_smallest_image_height(&images);
            let pen = HorizontalGridPen::new(image_count, 1, 1, smallest_height);
            return Box::new(pen);
        }
        if alignment == AlignmentMode::Vertical {
            let smallest_width = Self::find_smallest_image_width(&images);
            let pen = VerticalGridPen::new(image_count, 1, 1, smallest_width);
            return Box::new(pen);
        }

        // Find the smallest value "t" that satisfies t^2 >= count
        let mut t: usize = 1;
        while t * t < image_count {
            t += 1;
        }

        // Can now determine grid dimensions
        let grid_size_main_axis = t;
        let grid_size_cross_axis = image_count.div_ceil(t);
        let main_lines_with_full_size = image_count / t;

        // Get some stats on the aspect ratios present in the images
        let mut wide_count = 0;
        let mut portrait_count = 0;
        let mut squarish_count = 0;
        for img in images {
            let aspect_type = AspectType::get_aspect_from_dims(
                img.width(),
                img.height());
            match aspect_type {
                AspectType::Wide => wide_count += 1,
                AspectType::Portrait => portrait_count += 1,
                AspectType::Squarish => squarish_count += 1
            }
        }

        // Choose a drawing direction based on aspect ratios
        let draw_horizontal = portrait_count > wide_count || squarish_count >= wide_count;
        if draw_horizontal {
            let smallest_height = Self::find_smallest_image_height(&images);
            let pen = HorizontalGridPen::new(
                grid_size_main_axis,
                grid_size_cross_axis,
                main_lines_with_full_size,
                smallest_height
            );
            Box::new(pen)
        } else {
            let smallest_width = Self::find_smallest_image_width(&images);
            let pen = VerticalGridPen::new(
                grid_size_main_axis,
                grid_size_cross_axis,
                main_lines_with_full_size,
                smallest_width
            );
            Box::new(pen)
        }
    }

    fn find_smallest_image_width(images: &Vec<DynamicImage>) -> u32 {
        let mut smallest_size: u32 = 1024 * 1024;
        for image in images {
            smallest_size = smallest_size.min(image.width());
        }
        smallest_size
    }

    fn find_smallest_image_height(images: &Vec<DynamicImage>) -> u32 {
        let mut smallest_size: u32 = 1024 * 1024;
        for image in images {
            smallest_size = smallest_size.min(image.height());
        }
        smallest_size
    }

    pub fn stitch(self) -> Result<DynamicImage, String> {

        // Determine output file dimensions
        let out_dimensions = self.axis_pen.get_output_dimensions();

        // Create the image and paint individual images
        let mut output_image = DynamicImage::new_rgba8(out_dimensions.w, out_dimensions.h);
        for i in 0..self.images.len() {
            let img = &self.images[i];
            let rect = &self.image_rects[i];
            let scaled_image =
                img.resize_exact(rect.w, rect.h, image::imageops::Lanczos3);
            if let Err(err) = output_image.copy_from(&scaled_image, rect.x, rect.y) {
                return Err(format!("{} error while copying file #{}", err, i));
            }
        }

        Ok(output_image)
    }
}
