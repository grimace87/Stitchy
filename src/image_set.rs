extern crate image;

use image::GenericImageView;
use image::DynamicImage;
use self::image::{GenericImage, ImageOutputFormat};
use std::cmp::min;
use std::path::Path;
use std::time::SystemTime;
use crate::enums::{AlignmentMode, AspectType, Axis};
use std::fs::File;

pub struct FileData {
    pub full_path: String,
    pub modify_time: SystemTime
}

struct ImageRect {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

#[cfg(test)]
pub mod tests {
    use crate::enums::AlignmentMode;
    use crate::image_set::ImageSet;

    fn clear_output() -> Result<(), String> {
        let current_path = std::env::current_dir().unwrap();
        assert!(current_path.is_dir());
        let mut test_file = current_path.clone();
        test_file.push("test.jpg");
        return if test_file.is_file() {
            std::fs::remove_file(test_file.as_path()).map_err(
                |e| format!("Previous test file exists but couldn't be removed: {}", e)
            )
        } else {
            Ok(())
        }
    }

    #[test]
    pub fn test_types() {
        use std::path::Path;

        // Clear existing file
        let clear_result = clear_output();
        assert!(clear_result.is_ok(), clear_result.err().unwrap_or(String::new()));

        // Get files from test directory
        let retrieve_files_result =
            ImageSet::image_files_in_directory(vec!("images", "testing", "test_types"));
        assert!(retrieve_files_result.is_ok(), retrieve_files_result.err().unwrap_or(String::new()));

        // Process files, generate output
        let output_path = Path::new("./test.jpg");
        let image_files = retrieve_files_result.unwrap();
        let process_result: Result<(), String> =
            ImageSet::process_files(&output_path, 90usize, image_files, AlignmentMode::Grid, 0, 0);
        assert!(process_result.is_ok(), process_result.err().unwrap_or(String::new()));
    }

    #[test]
    pub fn test_sizes() {
        use std::path::Path;

        // Attempt increasing number of files, from 2 to 10
        for i in 2..11 {

            // Clear existing file
            let clear_result = clear_output();
            assert!(clear_result.is_ok(), clear_result.err().unwrap_or(String::new()));

            // Get files from test directory
            let retrieve_files_result = ImageSet::image_files_in_directory(vec!("images", "testing", "test_sizes"));
            assert!(retrieve_files_result.is_ok(), retrieve_files_result.err().unwrap_or(String::new()));

            // Use a subset of the images, as per the loop index
            let mut image_files = retrieve_files_result.unwrap();
            image_files.sort_unstable_by(|a, b| a.modify_time.cmp(&b.modify_time).reverse());
            image_files.truncate(i);
            image_files.sort_unstable_by(|a, b| a.modify_time.cmp(&b.modify_time));

            // Process files, generate output
            let output_path = Path::new("./test.jpg");
            let process_result: Result<(), String> = ImageSet::process_files(&output_path, 90usize, image_files, AlignmentMode::Grid, 0, 0);
            assert!(process_result.is_ok(), process_result.err().unwrap_or(String::new()));
        }
    }
}

pub struct ImageSet {
    images: Vec<image::DynamicImage>,
    alignment: AlignmentMode,
    width_limit: u32,
    height_limit: u32,
    is_prepared: bool,
    main_axis: Axis,
    grid_size_main_axis: u32,
    grid_size_cross_axis: u32,
    main_lines_with_full_size: u32,
    cross_axis_pixel_size_per_image: u32,
    image_rects: Vec<ImageRect>,
    largest_main_line_pixels: u32
}

impl ImageSet {

    /// Get all image files within a given directory
    pub fn image_files_in_directory(path_components: Vec<&str>) -> Result<Vec<FileData>, String> {

        // Get and verify current location
        let accepted_extensions: [&str; 5] = ["png", "jpg", "jpeg", "bmp", "gif"];
        let current_path = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return Err(String::from("Could not access current directory"))
        };
        if !current_path.is_dir() {
            return Err(String::from("Current directory cannot be opened as a directory"));
        }

        // Construct full path to locate images within
        let mut use_path = current_path;
        for component in path_components {
            use_path.push(component);
        }

        // Scan directory and add all image files found
        let mut image_files: Vec<FileData> = vec!();
        if use_path.is_dir() {
            for entry in std::fs::read_dir(use_path).unwrap() {

                // Check that the path is a file
                let path = entry.unwrap().path();
                if !path.is_file() {
                    continue;
                }

                // Get the 'stem' (file name before the extension), skip it if it looks like a
                // previous stitch
                let stem_not_stitch: Option<()> = (|| {
                    let file_stem = path
                        .file_stem()?
                        .to_str()?;
                    if file_stem.starts_with("stitch") {
                        None
                    } else {
                        Some(())
                    }
                })();
                if stem_not_stitch.is_none() {
                    continue;
                }

                // Get the extension and check it is accepted
                let extension_acceptable: Option<()> = (|| {
                    let extension = path
                        .extension()?
                        .to_str()?;
                    if !accepted_extensions.contains(&extension) {
                        None
                    } else {
                        Some(())
                    }
                })();
                if extension_acceptable.is_none() {
                    continue;
                }

                // Get path as str
                let path_str = match path.to_str() {
                    Some(path_as_str) => path_as_str,
                    None => continue
                };

                // Get file modify date from its metadata
                let modify_time: Result<SystemTime, String> = (|| {
                    path
                        .metadata()
                        .map_err(|_| format!("Failed reading metadata for: {}", path_str))?
                        .modified()
                        .map_err(|_| format!("Failed reading modify date for: {}", path_str))
                })();
                if modify_time.is_err() {
                    println!("{}", modify_time.unwrap_err());
                    continue;
                }

                // All seems well, push this file's properties into the vector
                let useful_data = FileData {
                    full_path: path_str.to_string(),
                    modify_time: modify_time.unwrap()
                };
                image_files.push(useful_data);
            }
        } else {
            return Err(format!("Requested path is not a directory:{}", use_path.to_str().unwrap()));
        }
        Ok(image_files)
    }

    /// Function accepting input images, processing them and creating the output.
    /// Designed to be unit-testable
    pub fn process_files(output_file_path: &Path, quality: usize, image_files: Vec<FileData>, alignment: AlignmentMode, width_limit: usize, height_limit: usize) -> Result<(), String> {

        // Decode all images and keep in memory for now
        let mut image_set = ImageSet::empty_set(alignment, width_limit, height_limit);
        for file in image_files {
            let path = Path::new(&file.full_path);
            if let Err(error) = image_set.add_from_file_path(path) {
                return Err(error);
            };
        }

        // Prepare data set before generating output
        image_set.generate_output_file(output_file_path, quality)
    }

    fn empty_set(alignment: AlignmentMode, width_limit: usize, height_limit: usize) -> ImageSet {
        ImageSet {
            images: Vec::new(),
            alignment,
            width_limit: width_limit as u32,
            height_limit: height_limit as u32,
            is_prepared: false,
            main_axis: Axis::Horizontal,
            grid_size_main_axis: 1,
            grid_size_cross_axis: 1,
            main_lines_with_full_size: 1,
            cross_axis_pixel_size_per_image: 1,
            image_rects: Vec::new(),
            largest_main_line_pixels: 1
        }
    }

    fn add_from_file_path(&mut self, path: &std::path::Path) -> Result<(), String> {
        let img: DynamicImage = match image::open(&path).ok() {
            Some(image) => image,
            None => return Err(String::from("Failed to open an image"))
        };
        let w = img.width();
        let h = img.height();
        self.images.push(img);
        if let Some(file_name) = path.file_name() {
            println!("Path: {}, w: {}, h: {}", file_name.to_str().unwrap(), w, h);
        }
        Ok(())
    }

    fn generate_output_file(&mut self, file_path: &Path, quality: usize) -> Result<(), String> {

        // Prepare if not already done
        if !self.is_prepared {
            self.prepare();
        }

        self.make_file(file_path, quality)
    }

    fn prepare(&mut self) {
        self.update_main_axis();
        self.update_grid_size();
        self.update_cross_axis_pixel_size();
        self.generate_sizing_metadata();
        self.check_size_limits();
        self.is_prepared = true;
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

        // Handle very particular alignments
        if self.alignment == AlignmentMode::Horizontal || self.alignment == AlignmentMode::Vertical {
            self.grid_size_main_axis = i;
            self.grid_size_cross_axis = 1;
            self.main_lines_with_full_size = 1;
            return;
        }

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
            let scaling_factor = (self.cross_axis_pixel_size_per_image as f64) / (image.height() as f64);
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
        self.cross_axis_pixel_size_per_image = (self.cross_axis_pixel_size_per_image as f64 * using_scale) as u32;
        self.largest_main_line_pixels = (self.largest_main_line_pixels as f64 * using_scale) as u32;
    }

    fn make_file(&self, file_path: &Path, quality: usize) -> Result<(), String> {
        if !self.is_prepared {
            panic!("Did not prepare image before attempting to save");
        }

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
            let scaled_image = img.resize_exact(rect.w, rect.h, image::imageops::Lanczos3);
            if let Err(err) = output_image.copy_from(&scaled_image, rect.x, rect.y) {
                return Err(format!("{} error while copying file #{}", err, i));
            }
        }

        // Save the file
        let mut file_writer = File::create(file_path).unwrap();
        match output_image.write_to(&mut file_writer, ImageOutputFormat::Jpeg(quality as u8)) {
            Ok(()) => Ok(()),
            Err(error) => Err(format!("Failed to generate output file - {}", error))
        }
    }
}
