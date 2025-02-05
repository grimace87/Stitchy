use crate::{FileLocation, FileProperties};
use image::{metadata::Orientation, DynamicImage, ImageFormat};

use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Wrapper for file metadata.
/// Loading these properties for a file does not require actually reading, let alone parsing, the
/// file's contents.
#[derive(Debug)]
pub struct FilePathWithMetadata {
    full_path: String,
    modify_time: SystemTime,
    size_bytes: u64,
}

impl Default for FilePathWithMetadata {
    fn default() -> Self {
        Self {
            full_path: "".to_owned(),
            modify_time: SystemTime::now(),
            size_bytes: 0,
        }
    }
}

impl FileProperties for FilePathWithMetadata {
    fn infer_format(&self) -> Option<ImageFormat> {
        match self.full_path.rfind('.') {
            Some(pos) => {
                let extension = &self.full_path[pos..self.full_path.len()];
                for &(ext, fmt) in crate::files::util::extension_formats().iter() {
                    if ext == extension {
                        return Some(fmt);
                    }
                }
                None
            }
            None => None,
        }
    }

    fn into_image_contents(self, print_info: bool) -> Result<DynamicImage, String> {
        let path = Path::new(&self.full_path);
        let image = image::open(path).map_err(|e| format!("Failed to open {:?}: {:?}", path, e))?;

        if print_info {
            if let Some(file_name) = path.file_name() {
                let w = image.width();
                let h = image.height();
                println!(
                    "Path: {}, w: {}, h: {}, {}",
                    file_name.to_str().unwrap(),
                    w,
                    h,
                    crate::util::make_size_string(self.size_bytes)
                );
            }
        }

        Ok(image)
    }

    #[inline]
    fn file_size(&self) -> u64 {
        self.size_bytes
    }

    #[inline]
    fn modify_time(&self) -> SystemTime {
        self.modify_time
    }

    #[inline]
    fn full_path(&self) -> Option<&String> {
        Some(&self.full_path)
    }

    fn orientation(&self) -> Result<Orientation, String> {
        let file = File::open(&self.full_path)
            .map_err(|e| format!("Cannot open file {}: {:?}", &self.full_path, e))?;
        self.decode_orientation(&file)
    }
}

/// Wrapper for a file's location by its absolute filesystem path
#[derive(Default, Debug)]
pub struct FilePath {
    path: PathBuf,
}

impl FilePath {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileLocation<FilePathWithMetadata> for FilePath {
    fn is_file(&self) -> Result<bool, String> {
        let path_buf = PathBuf::from(&self.path);
        Ok(path_buf.is_file())
    }

    fn extension(&self) -> Result<String, String> {
        let path_buf = PathBuf::from(&self.path);
        path_buf
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_ascii_lowercase()
            .into_string()
            .map_err(|o| format!("Error reading file extension, found {:?}", o))
    }

    fn into_properties(self) -> Result<FilePathWithMetadata, String> {
        // Get file size and modify date from its metadata
        let metadata = self
            .path
            .metadata()
            .map_err(|_| format!("Failed reading metadata for: {:?}", self.path))?;
        let modify_time = metadata
            .modified()
            .map_err(|_| format!("Failed reading modify date for: {:?}", self.path))?;
        let size_bytes = metadata.len();

        // All seems well, get this file's properties
        let path_string = match self.path.to_str() {
            Some(s) => s.to_owned(),
            None => return Err(format!("Failed converting to string: {:?}", self.path)),
        };
        let properties = FilePathWithMetadata {
            full_path: path_string,
            modify_time,
            size_bytes,
        };
        Ok(properties)
    }
}
