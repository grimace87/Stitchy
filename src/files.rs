
use crate::enums::ImageFormat;
use crate::options::Opt;
use std::time::SystemTime;
use std::fs::File;
use std::path::{Path, PathBuf};
use image::{DynamicImage, ImageOutputFormat};

pub struct ImageFiles {
    file_list: Vec<FileProperties>
}

pub struct FileProperties {
    full_path: String,
    modify_time: SystemTime
}

impl ImageFiles {

    /// Get all image files within a given directory
    pub fn from_directory(path_components: Vec<&str>) -> Result<ImageFiles, String> {

        // Get and verify current location
        let accepted_extensions = ImageFormat::allowed_extensions();
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
        let mut image_files: Vec<FileProperties> = vec!();
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
                let properties = FileProperties {
                    full_path: path_str.to_string(),
                    modify_time: modify_time.unwrap()
                };
                image_files.push(properties);
            }
        } else {
            return Err(format!("Requested path is not a directory:{}", use_path.to_str().unwrap()));
        }
        Ok(ImageFiles {
            file_list: image_files
        })
    }

    /// Sorts the files according to the options supplied, and truncates the set to the
    /// number of files requested by the user
    pub fn sort_and_truncate_by(mut self, options: &Opt) -> Result<Self, String> {

        // Get number of files requested, catch possible internal error
        let number_of_files = options.number_of_files.ok_or_else(|| String::from(
            "Internal error - sorting files before verifying that a number was supplied"))?;

        // Verify at least n images were found, where n is the number requested
        if self.file_list.len() < number_of_files {
            return Err(
                format!("Requested {} files, found {}", number_of_files, self.file_list.len()));
        }

        // Sort by ascending or descending alphabetical order, or by descending-order modify date, then take n.
        // Idea of 'naturally ordered' means that after taking n from start of the vector, the files will
        // be in the order in which they should be stitched, though the 'reverse' flag inverts this behaviour.
        let naturally_ordered: bool = if options.ascalpha {
            self.file_list.sort_unstable_by(|a, b| a.full_path.cmp(&b.full_path));
            true
        } else if options.descalpha {
            self.file_list.sort_unstable_by(|a, b| a.full_path.cmp(&b.full_path).reverse());
            true
        } else {
            self.file_list.sort_unstable_by(|a, b| a.modify_time.cmp(&b.modify_time).reverse());
            false
        };
        self.file_list.truncate(number_of_files);

        // Revert to chronological order, unless the reverse order was requested
        if options.reverse ^ !naturally_ordered {
            self.file_list.reverse();
        }

        // Return updated self
        Ok(self)
    }

    pub fn into_image_contents(self) -> Result<Vec<DynamicImage>, String> {
        let mut images = Vec::with_capacity(self.file_list.len());
        for file in self.file_list {

            let path = Path::new(&file.full_path);
            let image = image::open(&path)
                .map_err(|_| format!("Failed to open: {:?}", &path))?;

            // Print some info about source files at this point
            let w = image.width();
            let h = image.height();
            if let Some(file_name) = path.file_name() {
                println!("Path: {}, w: {}, h: {}", file_name.to_str().unwrap(), w, h);
            }

            // Collect values to return
            images.push(image);
        }

        Ok(images)
    }

    pub fn determine_output_format(&self, options: &Opt) -> Result<ImageFormat, String> {

        let mut image_format: ImageFormat = options.get_requested_image_format();

        // Check if no format was specified, but all source images are the same
        if image_format == ImageFormat::Unspecified {
            image_format = self.common_format_in_sources();
            if image_format == ImageFormat::Unspecified {
                image_format = ImageFormat::Jpeg;
            }
            if image_format != ImageFormat::Jpeg && options.quality != 100 {
                return Err(format!(
                    "Output file with extension .{} cannot use a quality setting.",
                    image_format.get_main_extension()));
            }
        }

        Ok(image_format)
    }

    fn common_format_in_sources(&self) -> ImageFormat {
        if self.file_list.is_empty() {
            return ImageFormat::Unspecified;
        }
        let mut all_formats = self.file_list.iter().map(|file_data| {
            ImageFormat::infer_format(&file_data.full_path)
        });
        let first_format = all_formats.next().unwrap();
        match all_formats.all(|fmt| fmt == first_format) {
            true => first_format,
            false => ImageFormat::Unspecified
        }
    }

    pub fn next_available_output(&self, options: &Opt) -> Result<PathBuf, String> {

        let target_extension = self.determine_output_format(options)?
            .get_main_extension();

        // Get current path, check if the default file name exists, if not return it
        let mut current_path: PathBuf = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return Err(String::from("Could not access current directory"))
        };
        let mut un_numbered_file_exists = false;
        for &extension in ImageFormat::allowed_extensions().iter() {
            current_path.push(format!("stitch.{}", extension));
            if current_path.is_file() {
                un_numbered_file_exists = true;
                current_path.pop();
                break;
            }
            current_path.pop();
        }
        if !un_numbered_file_exists {
            let mut path = current_path.clone();
            path.push(format!("stitch.{}", target_extension));
            return Ok(path);
        }

        // Check file names until a usable one is found
        let mut i = 1usize;
        while i < 1000 {
            let mut numbered_file_exists = false;
            for &extension in ImageFormat::allowed_extensions().iter() {
                let file_name: String = format!("stitch_{}.{}", i, extension);
                current_path.push(file_name);
                if current_path.is_file() {
                    numbered_file_exists = true;
                }
                current_path.pop();
                if numbered_file_exists {
                    break;
                }
            }
            if !numbered_file_exists {
                let mut path = current_path.clone();
                path.push(format!("stitch_{}.{}", i, target_extension));
                return Ok(path);
            }
            i += 1;
        };
        Err(String::from("Did not find a usable file name - if you have 1000 stitches, please move or delete some."))
    }
}

pub fn write_image_to_file(image: DynamicImage, file_path: &Path, format: ImageFormat, quality: usize) -> Result<(), String> {
    let mut file_writer = File::create(file_path).unwrap();
    let format = match format {
        ImageFormat::Jpeg => ImageOutputFormat::Jpeg(quality as u8),
        ImageFormat::Png => ImageOutputFormat::Png,
        ImageFormat::Gif => ImageOutputFormat::Gif,
        ImageFormat::Bmp => ImageOutputFormat::Bmp,
        ImageFormat::Unspecified => ImageOutputFormat::Jpeg(100u8) // Should not reach this point
    };
    match image.write_to(&mut file_writer, format) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("Failed to generate output file - {}", error))
    }
}
