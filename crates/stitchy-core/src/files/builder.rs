
use crate::{ImageFiles, ImageFormat, files::FileProperties};
use std::ffi::OsStr;
use std::path::PathBuf;

/// Builder for the [`ImageFiles`] struct.
///
/// Assemble a list of image files, chosen individually or a directory at a time.
///
/// See documentation for [`ImageFiles`] for more information.
pub struct ImageFilesBuilder {
    file_list: Vec<PathBuf>
}

impl Default for ImageFilesBuilder {
    fn default() -> Self {
        Self {
            file_list: vec![]
        }
    }
}

impl ImageFilesBuilder {

    /// Return an [ImageFiles] containing the metadata of the set of source files.
    ///
    /// If there was an error reading the metadata of any file, it will be returned as a String.
    pub fn build(self) -> Result<ImageFiles, String> {
        let mut image_files: Vec<FileProperties> = vec!();
        for path in self.file_list.into_iter() {

            // Get file modify date from its metadata
            let (modify_time, size_bytes) = {
                let metadata = path
                    .metadata()
                    .map_err(|_| format!("Failed reading metadata for: {:?}", path))?;
                let time_result = metadata
                    .modified()
                    .map_err(|_| format!("Failed reading modify date for: {:?}", path));
                (time_result, metadata.len())
            };
            if modify_time.is_err() {
                println!("{}", modify_time.unwrap_err());
                continue;
            }

            // All seems well, push this file's properties into the vector
            let path_str = match path.to_str() {
                Some(path_as_str) => path_as_str,
                None => return Err(format!("Failed interpreting path: {:?}", path))
            };
            let properties = FileProperties {
                full_path: path_str.to_string(),
                modify_time: modify_time.unwrap(),
                size_bytes
            };
            image_files.push(properties);
        }
        Ok(ImageFiles::new(image_files))
    }

    /// Add a single file to the working set, given a PathBuf with its absolute path.
    pub fn add_file(mut self, path: PathBuf) -> Result<Self, String> {
        let accepted_extensions = ImageFormat::allowed_extensions();
        if !Self::extension_in_list(&path, &accepted_extensions) {
            return Err(format!("File not recognised as image file: {:?}", path));
        }
        if !path.is_file() {
            return Err(format!("Path is not a file: {:?}", path));
        }
        self.file_list.push(path);
        Ok(self)
    }

    /// Add to the working set all files within the current directory that have known image file
    /// extensions.
    pub fn add_current_directory(self, additional_components: Vec<&str>) -> Result<Self, String> {

        // Get and verify current location
        let current_path = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => return Err(String::from("Could not access current directory"))
        };
        if !current_path.is_dir() {
            return Err(String::from("Current directory cannot be opened as a directory"));
        }

        // Construct full path to locate images within
        let mut use_path = current_path;
        for component in additional_components {
            use_path.push(component);
        }

        self.add_directory(use_path)
    }

    /// Add to the working set all files within the given directory that have known image file
    /// extensions. The supplied PathBuf must be the absolute path to a directory.
    pub fn add_directory(mut self, source_path: PathBuf) -> Result<Self, String> {

        // Scan directory and add all image files found
        let accepted_extensions = ImageFormat::allowed_extensions();
        let mut image_files: Vec<PathBuf> = vec!();
        if source_path.is_dir() {
            for entry in std::fs::read_dir(source_path).unwrap() {

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

                // Check the extension is a known image format
                if !Self::extension_in_list(&path, &accepted_extensions) {
                    continue;
                }

                // Add to list of usable paths
                image_files.push(path);
            }
        } else {
            return Err(
                format!("Requested path is not a directory:{}", source_path.to_str().unwrap()));
        }

        self.file_list.append(&mut image_files);
        Ok(self)
    }

    /// Checks if a file has an extension matching any in a given set
    fn extension_in_list(file_path: &PathBuf, accepted_extensions: &[&str; 5]) -> bool {
        let extension = file_path.extension()
            .unwrap_or(OsStr::new(""))
            .to_ascii_lowercase();
        let lower_str_extension = extension
            .to_str()
            .unwrap_or("");

        accepted_extensions.contains(&lower_str_extension)
    }
}
