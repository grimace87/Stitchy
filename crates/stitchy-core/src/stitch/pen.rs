use image::DynamicImage;
use std::cmp::min;

/// Size and position of an area within an image
pub(crate) struct ImageRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Size of an image
pub(crate) struct ImageSize {
    pub w: u32,
    pub h: u32,
}

impl ImageSize {
    pub fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
}

/// Trait whose implementors can trace out an image grid, including the dimensions therein
pub(crate) trait ImageGridPen {
    #[cfg(test)]
    fn get_images_per_line(&self) -> usize;

    #[cfg(test)]
    fn get_line_count(&self) -> usize;

    #[cfg(test)]
    fn get_lines_at_full_size(&self) -> usize;

    fn get_output_dimensions(&self) -> ImageSize;
    fn generate_output_rects(&mut self, images: &Vec<DynamicImage>) -> Vec<ImageRect>;
    fn scale_image_rects(
        &mut self,
        image_rects: Vec<ImageRect>,
        width_limit: u32,
        height_limit: u32,
    ) -> Vec<ImageRect>;
}

/// Type that knows how to trace an image grid from left to right, then down across lines
pub(crate) struct HorizontalGridPen {
    line_length: usize,
    line_count: usize,
    lines_at_full_size: usize,
    longest_line_length_pixels: u32,
    line_size_pixels: u32,
}

impl HorizontalGridPen {
    pub fn new(
        line_length: usize,
        line_count: usize,
        lines_at_full_size: usize,
        line_size_pixels: u32,
    ) -> Self {
        Self {
            line_length,
            line_count,
            lines_at_full_size,
            longest_line_length_pixels: 0,
            line_size_pixels,
        }
    }
}

impl ImageGridPen for HorizontalGridPen {
    #[cfg(test)]
    fn get_images_per_line(&self) -> usize {
        self.line_length
    }

    #[cfg(test)]
    fn get_line_count(&self) -> usize {
        self.line_count
    }

    #[cfg(test)]
    fn get_lines_at_full_size(&self) -> usize {
        self.lines_at_full_size
    }

    fn get_output_dimensions(&self) -> ImageSize {
        ImageSize::new(
            self.longest_line_length_pixels,
            self.line_size_pixels * self.line_count as u32,
        )
    }

    fn generate_output_rects(&mut self, images: &Vec<DynamicImage>) -> Vec<ImageRect> {
        let mut image_rects: Vec<ImageRect> = vec![];
        let mut pen_x: u32 = 0;
        let mut pen_y: u32 = 0;
        let mut grid_x: usize = 0;
        let mut grid_y: usize = 0;

        for image in images {
            // Get sizing for this image
            let scaling_factor = (self.line_size_pixels as f64) / (image.height() as f64);
            let scaled_width = ((image.width() as f64) * scaling_factor) as u32;
            image_rects.push(ImageRect {
                x: pen_x,
                y: pen_y,
                w: scaled_width,
                h: self.line_size_pixels,
            });

            // Advance pen and grid positions
            pen_x += scaled_width;
            grid_x += 1;
            if (grid_y <= self.lines_at_full_size) && (grid_x >= self.line_length) {
                if self.longest_line_length_pixels < pen_x {
                    self.longest_line_length_pixels = pen_x;
                }
                pen_x = 0;
                pen_y += self.line_size_pixels;
                grid_x = 0;
                grid_y += 1;
            }
        }

        // Check if the pen overshot the known boundary before filling a grid row
        if self.longest_line_length_pixels < pen_x {
            self.longest_line_length_pixels = pen_x;
        }

        image_rects
    }

    fn scale_image_rects(
        &mut self,
        image_rects: Vec<ImageRect>,
        width_limit: u32,
        height_limit: u32,
    ) -> Vec<ImageRect> {
        let total_width: u32;
        let allowed_width: u32;
        let total_height: u32;
        let allowed_height: u32;

        total_width = self.longest_line_length_pixels;
        allowed_width = if width_limit == 0 {
            total_width
        } else {
            min(total_width, width_limit)
        };

        // Height is cross axis
        total_height = (self.line_count as u32) * self.line_size_pixels;
        allowed_height = if height_limit == 0 {
            total_height
        } else {
            min(total_height, height_limit)
        };

        // No scaling needed if limits exceed current size
        if total_width <= allowed_width && total_height <= allowed_height {
            return image_rects;
        }

        // Reset the longest known line sizes; they will be updated
        self.longest_line_length_pixels = 0;
        self.line_size_pixels = 0;

        // Scale according to the greatest necessary reduction
        let width_scale = allowed_width as f64 / total_width as f64;
        let height_scale = allowed_height as f64 / total_height as f64;
        let using_scale: f64 = f64::min(width_scale, height_scale);

        // For each image, downscale its position and size
        let mut pen_x: f64 = 0.0;
        let mut pen_y: f64 = 0.0;
        let mut next_pen_x: f64;
        let mut next_pen_y: f64;
        let mut image_on_line: usize = 0;
        let mut scaled_rects = vec![];
        for rect in image_rects.into_iter() {
            let w = rect.w as f64 * using_scale;
            let h = rect.h as f64 * using_scale;
            next_pen_x = pen_x + w;
            next_pen_y = pen_y + h;
            let scaled_rect = ImageRect {
                x: pen_x.round() as u32,
                y: pen_y.round() as u32,
                w: (next_pen_x.round() as u32) - (pen_x.round() as u32),
                h: (next_pen_y.round() as u32) - (pen_y.round() as u32),
            };
            if self.line_size_pixels < scaled_rect.h {
                self.line_size_pixels = scaled_rect.h;
            }
            if self.longest_line_length_pixels < next_pen_x.round() as u32 {
                self.longest_line_length_pixels = next_pen_x.round() as u32;
            }

            scaled_rects.push(scaled_rect);

            image_on_line += 1;
            if image_on_line >= self.line_length {
                image_on_line = 0;
                pen_x = 0.0;
                pen_y = next_pen_y;
            } else {
                pen_x = next_pen_x;
            }
        }

        scaled_rects
    }
}

/// Type that knows how to trace an image grid from top to bottom, then right across lines
pub(crate) struct VerticalGridPen {
    line_length: usize,
    line_count: usize,
    lines_at_full_size: usize,
    longest_line_length_pixels: u32,
    line_size_pixels: u32,
}

impl VerticalGridPen {
    pub fn new(
        line_length: usize,
        line_count: usize,
        lines_at_full_size: usize,
        line_size_pixels: u32,
    ) -> Self {
        Self {
            line_length,
            line_count,
            lines_at_full_size,
            longest_line_length_pixels: 0,
            line_size_pixels,
        }
    }
}

impl ImageGridPen for VerticalGridPen {
    #[cfg(test)]
    fn get_images_per_line(&self) -> usize {
        self.line_length
    }

    #[cfg(test)]
    fn get_line_count(&self) -> usize {
        self.line_count
    }

    #[cfg(test)]
    fn get_lines_at_full_size(&self) -> usize {
        self.lines_at_full_size
    }

    fn get_output_dimensions(&self) -> ImageSize {
        ImageSize::new(
            self.line_size_pixels * self.line_count as u32,
            self.longest_line_length_pixels,
        )
    }

    fn generate_output_rects(&mut self, images: &Vec<DynamicImage>) -> Vec<ImageRect> {
        let mut image_rects: Vec<ImageRect> = vec![];
        let mut pen_x: u32 = 0;
        let mut pen_y: u32 = 0;
        let mut grid_x: usize = 0;
        let mut grid_y: usize = 0;

        for image in images {
            // Get sizing for this image
            let scaling_factor = (self.line_size_pixels as f64) / (image.width() as f64);
            let scaled_height = ((image.height() as f64) * scaling_factor) as u32;
            image_rects.push(ImageRect {
                x: pen_x,
                y: pen_y,
                w: self.line_size_pixels,
                h: scaled_height,
            });

            // Advance pen and grid positions
            pen_y += scaled_height;
            grid_y += 1;
            if (grid_x <= self.lines_at_full_size) && (grid_y >= self.line_length) {
                if self.longest_line_length_pixels < pen_y {
                    self.longest_line_length_pixels = pen_y;
                }
                pen_y = 0;
                pen_x += self.line_size_pixels;
                grid_y = 0;
                grid_x += 1;
            }
        }

        // Check if the pen overshot the known boundary before filling a grid column
        if self.longest_line_length_pixels < pen_y {
            self.longest_line_length_pixels = pen_y;
        }

        image_rects
    }

    fn scale_image_rects(
        &mut self,
        image_rects: Vec<ImageRect>,
        width_limit: u32,
        height_limit: u32,
    ) -> Vec<ImageRect> {
        let total_width: u32;
        let allowed_width: u32;
        let total_height: u32;
        let allowed_height: u32;

        total_height = self.longest_line_length_pixels;
        allowed_height = if height_limit == 0 {
            total_height
        } else {
            min(total_height, height_limit)
        };

        // Width is cross axis
        total_width = (self.line_count as u32) * self.line_size_pixels;
        allowed_width = if width_limit == 0 {
            total_width
        } else {
            min(total_width, width_limit)
        };

        // No scaling needed if limits exceed current size
        if total_width <= allowed_width && total_height <= allowed_height {
            return image_rects;
        }

        // Reset the longest known line sizes; they will be updated
        self.longest_line_length_pixels = 0;
        self.line_size_pixels = 0;

        // Scale according to the greatest necessary reduction
        let width_scale = allowed_width as f64 / total_width as f64;
        let height_scale = allowed_height as f64 / total_height as f64;
        let using_scale: f64 = f64::min(width_scale, height_scale);

        // For each image, downscale its position and size
        let mut pen_y: f64 = 0.0;
        let mut pen_x: f64 = 0.0;
        let mut next_pen_y: f64;
        let mut next_pen_x: f64;
        let mut image_on_line: usize = 0;
        let mut scaled_rects = vec![];
        for rect in image_rects.into_iter() {
            let h = rect.h as f64 * using_scale;
            let w = rect.w as f64 * using_scale;
            next_pen_y = pen_y + h;
            next_pen_x = pen_x + w;
            let scaled_rect = ImageRect {
                x: pen_x.round() as u32,
                y: pen_y.round() as u32,
                w: (next_pen_x.round() as u32) - (pen_x.round() as u32),
                h: (next_pen_y.round() as u32) - (pen_y.round() as u32),
            };
            if self.line_size_pixels < scaled_rect.w {
                self.line_size_pixels = scaled_rect.w;
            }
            if self.longest_line_length_pixels < next_pen_y.round() as u32 {
                self.longest_line_length_pixels = next_pen_y.round() as u32;
            }

            scaled_rects.push(scaled_rect);

            image_on_line += 1;
            if image_on_line >= self.line_length {
                image_on_line = 0;
                pen_x = next_pen_x;
                pen_y = 0.0;
            } else {
                pen_y = next_pen_y;
            }
        }

        scaled_rects
    }
}
