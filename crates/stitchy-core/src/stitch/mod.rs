pub mod builder;

#[cfg(test)]
mod tests;

use crate::{StitchBuilder, image::{DynamicImage, GenericImage}};
use std::cmp::min;

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

/// Direction of either the main or cross axis
#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Axis {
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

/// Size and position of an area within an image
struct ImageRect {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

/// The full set of inputs for a stitch operation, including the source images and the layout
/// that the output will take. Use the [StitchBuilder] for the entire stitching process.
pub struct Stitch {
    images: Vec<DynamicImage>,
    alignment: AlignmentMode,
    width_limit: u32,
    height_limit: u32,
    main_axis: Axis,
    grid_size_main_axis: u32,
    grid_size_cross_axis: u32,
    main_lines_with_full_size: u32,
    cross_axis_pixel_size_per_image: u32,
    image_rects: Vec<ImageRect>,
    largest_main_line_pixels: u32
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

        let mut set = Stitch {
            images,
            alignment,
            width_limit,
            height_limit,
            main_axis: Axis::Horizontal,
            grid_size_main_axis: 1,
            grid_size_cross_axis: 1,
            main_lines_with_full_size: 1,
            cross_axis_pixel_size_per_image: 1,
            image_rects: Vec::new(),
            largest_main_line_pixels: 1
        };

        set.update_main_axis();
        set.update_grid_size();
        set.update_cross_axis_pixel_size();
        set.generate_sizing_metadata();
        set.check_size_limits();

        set
    }

    /// Sets main_axis
    fn update_main_axis(&mut self) {

        // Check for very particular alignment modes
        if self.alignment == AlignmentMode::Horizontal {
            self.main_axis = Axis::Horizontal;
            return
        }
        if self.alignment == AlignmentMode::Vertical {
            self.main_axis = Axis::Vertical;
            return;
        }

        let mut wide_count = 0;
        let mut portrait_count = 0;
        let mut squarish_count = 0;
        for img in &self.images {
            let aspect_type = AspectType::get_aspect_from_dims(
                img.width(),
                img.height());
            match aspect_type {
                AspectType::Wide => wide_count += 1,
                AspectType::Portrait => portrait_count += 1,
                AspectType::Squarish => squarish_count += 1
            }
        }

        self.main_axis = if wide_count > portrait_count && wide_count >= squarish_count {
            Axis::Vertical
        } else {
            Axis::Horizontal
        }
    }

    /// Sets grid_size_main_axis, grid_size_cross_axis and main_lines_with_full_size
    fn update_grid_size(&mut self) {
        let count = self.images.len() as u32;

        // Handle very particular alignments
        if self.alignment == AlignmentMode::Horizontal || self.alignment == AlignmentMode::Vertical {
            self.grid_size_main_axis = count;
            self.grid_size_cross_axis = 1;
            self.main_lines_with_full_size = 1;
            return;
        }

        // Find the smallest value 't' that satisfies t^2 >= count
        let mut t: u32 = 1;
        while t * t < count {
            t += 1;
        }

        // Can now determine sizes of axes
        self.grid_size_main_axis = t;
        self.grid_size_cross_axis = count.div_ceil(t);
        self.main_lines_with_full_size = count / t;
    }

    /// Sets cross_axis_pixel_size_per_image
    fn update_cross_axis_pixel_size(&mut self) {
        let mut smallest_size: u32 = 1024 * 1024;
        for image in &self.images {
            if self.main_axis == Axis::Horizontal {
                if image.height() < smallest_size {
                    smallest_size = image.height();
                }
            } else if image.width() < smallest_size {
                smallest_size = image.width();
            }
        }
        self.cross_axis_pixel_size_per_image = smallest_size;
    }

    /// Generates sizing_metadata and sets largest_main_line_pixels
    fn generate_sizing_metadata(&mut self) {
        if !self.image_rects.is_empty() {
            return;
        }
        if self.main_axis == Axis::Horizontal {
            self.generate_sizing_metadata_with_main_axis_horizontal();
        } else {
            self.generate_sizing_metadata_with_main_axis_vertical();
        }
    }

    fn generate_sizing_metadata_with_main_axis_horizontal(&mut self) {
        self.largest_main_line_pixels = 0;
        let mut pen_x: u32 = 0;
        let mut pen_y: u32 = 0;
        let mut grid_x: u32 = 0;
        let mut grid_y: u32 = 0;

        for image in &self.images {

            // Get sizing for this image
            let scaling_factor =
                (self.cross_axis_pixel_size_per_image as f64) / (image.height() as f64);
            let scaled_width = ((image.width() as f64) * scaling_factor) as u32;
            self.image_rects.push(ImageRect {
                x: pen_x,
                y: pen_y,
                w: scaled_width,
                h: self.cross_axis_pixel_size_per_image
            });

            // Advance pen and grid positions
            pen_x += scaled_width;
            grid_x += 1;
            if (grid_y <= self.main_lines_with_full_size) && (grid_x >= self.grid_size_main_axis) {
                if self.largest_main_line_pixels < pen_x {
                    self.largest_main_line_pixels = pen_x;
                }
                pen_x = 0;
                pen_y += self.cross_axis_pixel_size_per_image;
                grid_x = 0;
                grid_y += 1;
            }
        }

        // Check if the pen overshot the known boundary before filling a grid row
        if self.largest_main_line_pixels < pen_x {
            self.largest_main_line_pixels = pen_x;
        }
    }

    fn generate_sizing_metadata_with_main_axis_vertical(&mut self) {
        self.largest_main_line_pixels = 0;
        let mut pen_x: u32 = 0;
        let mut pen_y: u32 = 0;
        let mut grid_x: u32 = 0;
        let mut grid_y: u32 = 0;

        for image in &self.images {

            // Get sizing for this image
            let scaling_factor =
                (self.cross_axis_pixel_size_per_image as f64) / (image.width() as f64);
            let scaled_height = ((image.height() as f64) * scaling_factor) as u32;
            self.image_rects.push(ImageRect {
                x: pen_x,
                y: pen_y,
                w: self.cross_axis_pixel_size_per_image,
                h: scaled_height
            });

            // Advance pen and grid positions
            pen_y += scaled_height;
            grid_y += 1;
            if (grid_x <= self.main_lines_with_full_size) && (grid_y >= self.grid_size_main_axis) {
                if self.largest_main_line_pixels < pen_y {
                    self.largest_main_line_pixels = pen_y;
                }
                pen_y = 0;
                pen_x += self.cross_axis_pixel_size_per_image;
                grid_y = 0;
                grid_x += 1;
            }
        }

        // Check if the pen overshot the known boundary before filling a grid column
        if self.largest_main_line_pixels < pen_y {
            self.largest_main_line_pixels = pen_y;
        }
    }

    /// Check if the image will exceed the limits applied, and scale down
    /// if need be
    fn check_size_limits(&mut self) {

        // No adjustments if no limits were placed
        if self.width_limit == 0 && self.height_limit == 0 {
            return;
        }

        let total_width: u32;
        let allowed_width: u32;
        let total_height: u32;
        let allowed_height: u32;
        if self.main_axis == Axis::Horizontal {

            // Width is main axis
            total_width = self.largest_main_line_pixels;
            allowed_width = if self.width_limit == 0 {
                total_width
            } else {
                min(total_width, self.width_limit)
            };

            // Height is cross axis
            total_height = self.grid_size_cross_axis * self.cross_axis_pixel_size_per_image;
            allowed_height = if self.height_limit == 0 {
                total_height
            } else {
                min(total_height, self.height_limit)
            };
        } else {

            // Height is main axis
            total_height = self.largest_main_line_pixels;
            allowed_height = if self.height_limit == 0 {
                total_height
            } else {
                min(total_height, self.height_limit)
            };

            // Width is cross axis
            total_width = self.grid_size_cross_axis * self.cross_axis_pixel_size_per_image;
            allowed_width = if self.width_limit == 0 {
                total_width
            } else {
                min(total_width, self.width_limit)
            };
        }

        // No scaling needed if limits exceed current size
        if total_width <= allowed_width && total_height <= allowed_height {
            return;
        }

        // Scale according to the greatest necessary reduction
        let width_scale = allowed_width as f64 / total_width as f64;
        let height_scale = allowed_height as f64 / total_height as f64;
        let using_scale: f64 = f64::min(width_scale, height_scale);

        // For each image, downscale its position and size
        for sizing_data in self.image_rects.iter_mut() {
            sizing_data.w = (sizing_data.w as f64 * using_scale) as u32;
            sizing_data.h = (sizing_data.h as f64 * using_scale) as u32;
            sizing_data.x = (sizing_data.x as f64 * using_scale) as u32;
            sizing_data.y = (sizing_data.y as f64 * using_scale) as u32;
        }

        // Update output image pixel sizes
        self.cross_axis_pixel_size_per_image =
            (self.cross_axis_pixel_size_per_image as f64 * using_scale) as u32;
        self.largest_main_line_pixels = (self.largest_main_line_pixels as f64 * using_scale) as u32;
    }

    pub fn stitch(self) -> Result<DynamicImage, String> {

        // Determine output file dimensions
        let out_w = match self.main_axis {
            Axis::Horizontal => self.largest_main_line_pixels,
            _ => self.cross_axis_pixel_size_per_image * self.grid_size_cross_axis
        };
        let out_h = match self.main_axis {
            Axis::Horizontal => self.cross_axis_pixel_size_per_image * self.grid_size_cross_axis,
            _ => self.largest_main_line_pixels
        };

        // Create the image and paint individual images
        let mut output_image = DynamicImage::new_rgba8(out_w, out_h);
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
