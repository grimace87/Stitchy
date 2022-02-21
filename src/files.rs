
use crate::enums::ImageFormat;
use std::time::SystemTime;

pub struct FileData {
    pub full_path: String,
    pub modify_time: SystemTime
}

impl FileData {

    /// Get all image files within a given directory
    pub fn image_files_in_directory(path_components: Vec<&str>) -> Result<Vec<FileData>, String> {

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
}
