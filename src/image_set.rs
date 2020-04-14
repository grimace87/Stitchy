
pub mod image_set {
    extern crate image;
    use image::GenericImageView;
    use image::DynamicImage;
    use self::image::GenericImage;
    use std::path::Path;

    pub struct ImageSet {
        images: Vec<image::DynamicImage>,
        is_prepared: bool,
        main_axis: Axis,
        grid_size_main_axis: u32,
        grid_size_cross_axis: u32,
        main_lines_with_full_size: u32,
        cross_axis_pixel_size_per_image: u32,
        sizing_metadata: Vec<SizingMetadata>,
        largest_main_line_pixels: u32
    }

    impl ImageSet {
        pub fn empty_set() -> ImageSet {
            ImageSet {
                images: Vec::new(),
                is_prepared: false,
                main_axis: Axis::Horizontal,
                grid_size_main_axis: 1,
                grid_size_cross_axis: 1,
                main_lines_with_full_size: 1,
                cross_axis_pixel_size_per_image: 1,
                sizing_metadata: Vec::new(),
                largest_main_line_pixels: 1
            }
        }

        pub fn add_from_file_path(&mut self, path: &std::path::Path) {
            let img: DynamicImage = image::open(&path).ok().expect("Failed to open an image");
            let w = img.width();
            let h = img.height();
            self.images.push(img);
            println!("Path: {}, w: {}, h: {}", path.file_name().unwrap().to_str().unwrap(), w, h);
        }

        pub fn generate_output_file(&mut self, file_path: &Path) {

            // Prepare if not already done
            if !self.is_prepared {
                self.prepare();
            }

            self.make_file(file_path);
        }

        fn prepare(&mut self) {
            self.update_main_axis();
            self.update_grid_size();
            self.update_cross_axis_pixel_size();
            self.generate_sizing_metadata();
            self.is_prepared = true;
        }

        /// Sets main_axis
        fn update_main_axis(&mut self) {
            let mut wide_count = 0;
            let mut portrait_count = 0;
            let mut squarish_count = 0;
            for img in &self.images {
                let aspect_type = AspectType::get_aspect_from_dims(img.width(), img.height());
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
            let i = self.images.len() as  u32;

            // Handle special case where i is 2 or 3
            if i < 4 {
                self.grid_size_main_axis = i;
                self.grid_size_cross_axis = 1;
                self.main_lines_with_full_size = 1;
                return;
            }

            // Find the largest value 't' that satisfies (t - 1)^2 < i
            let mut t: u32 = 1;
            while (t - 1) * (t - 1) < i {
                t += 1;
            }
            t -= 1;

            // Can now determine sizes of axes
            self.grid_size_main_axis = t;
            if i <= (t * (t - 1)) {
                self.grid_size_cross_axis = t - 1;
                self.main_lines_with_full_size = i - (t - 1) * (t - 1);
            } else {
                self.grid_size_cross_axis = t;
                self.main_lines_with_full_size = i - t * (t - 1);
            }
        }

        /// Sets cross_axis_pixel_size_per_image
        fn update_cross_axis_pixel_size(&mut self) {
            let mut smallest_size: u32 = 1024 * 1024;
            for image in &self.images {
                if self.main_axis == Axis::Horizontal {
                    if image.height() < smallest_size {
                        smallest_size = image.height();
                    }
                } else {
                    if image.width() < smallest_size {
                        smallest_size = image.width();
                    }
                }
            }
            self.cross_axis_pixel_size_per_image = smallest_size;
        }

        /// Generates sizing_metadata and sets largest_main_line_pixels
        fn generate_sizing_metadata(&mut self) {
            if !self.sizing_metadata.is_empty() {
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
                let scaling_factor = (self.cross_axis_pixel_size_per_image as f64) / (image.height() as f64);
                let scaled_width = ((image.width() as f64) * scaling_factor) as u32;
                self.sizing_metadata.push(SizingMetadata {
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
        }

        fn generate_sizing_metadata_with_main_axis_vertical(&mut self) {
            self.largest_main_line_pixels = 0;
            let mut pen_x: u32 = 0;
            let mut pen_y: u32 = 0;
            let mut grid_x: u32 = 0;
            let mut grid_y: u32 = 0;

            for image in &self.images {

                // Get sizing for this image
                let scaling_factor = (self.cross_axis_pixel_size_per_image as f64) / (image.width() as f64);
                let scaled_height = ((image.height() as f64) * scaling_factor) as u32;
                self.sizing_metadata.push(SizingMetadata {
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
        }

        fn make_file(&self, file_path: &Path) {
            if !self.is_prepared {
                panic!("Did not prepare image before attempting to save");
            }

            // Determine output file dimensions
            let out_w;
            let out_h;
            if self.main_axis == Axis::Horizontal {
                out_w = self.largest_main_line_pixels;
                out_h = self.cross_axis_pixel_size_per_image * self.grid_size_cross_axis;
            } else {
                out_w = self.cross_axis_pixel_size_per_image * self.grid_size_cross_axis;
                out_h = self.largest_main_line_pixels;
            }

            // Create the image and paint individual images
            let mut output_image = DynamicImage::new_rgba8(out_w, out_h);
            for i in 0..self.images.len() {
                let img = &self.images[i];
                let metadata = &self.sizing_metadata[i];
                let scaled_image = img.resize_exact(metadata.w, metadata.h, image::imageops::Lanczos3);
                output_image.copy_from(&scaled_image, metadata.x, metadata.y);
            }

            // Save the file
            output_image.save(file_path).unwrap();
        }
    }

    enum AspectType {
        Wide,
        Portrait,
        Squarish
    }

    impl AspectType {
        fn get_aspect_from_dims(w: u32, h: u32) -> AspectType {
            let aspect_ratio: f32 = (w as f32) / (h as f32);
            return if aspect_ratio > 1.25f32 {
                AspectType::Wide
            } else if aspect_ratio < 0.8f32 {
                AspectType::Portrait
            } else {
                AspectType::Squarish
            }
        }
    }

    #[derive(PartialEq)]
    enum Axis {
        Horizontal,
        Vertical
    }

    struct SizingMetadata {
        x: u32,
        y: u32,
        w: u32,
        h: u32
    }
}
