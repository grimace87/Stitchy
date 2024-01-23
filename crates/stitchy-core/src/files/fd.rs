
use crate::{FileLocation, FileProperties};
use image::{DynamicImage, ImageFormat};
use std::fs::File;
use std::io::Read;
use std::os::fd::{FromRawFd, RawFd};
use std::time::SystemTime;

/// Wrapper for file metadata.
/// Loading these properties for a file does not require actually reading, let alone parsing, the
/// file's contents.
/// Like the [OwnedRawFdLocation], this owns the file descriptor now, and the file will be closed when
/// this instance is dropped.
#[derive(Debug)]
pub struct OwnedRawFdProperties {
    fd: RawFd,
    file: File,
    mime_type: String,
    modify_time: SystemTime,
    size_bytes: u64
}

impl OwnedRawFdProperties {
    pub fn borrow_file_mut(&mut self) -> &File {
        &mut self.file
    }
}

impl FileProperties for OwnedRawFdProperties {

    fn infer_format(&self) -> Option<ImageFormat> {
        match self.mime_type.as_str() {
            "image/jpeg" => Some(ImageFormat::Jpeg),
            "image/png" => Some(ImageFormat::Png),
            "image/gif" => Some(ImageFormat::Gif),
            "image/bmp" => Some(ImageFormat::Bmp),
            _ => None
        }
    }

    fn into_image_contents(mut self, print_info: bool) -> Result<DynamicImage, String> {
        let mut image_buffer = Vec::new();
        self.file
            .read_to_end(&mut image_buffer)
            .map_err(|e| format!("Failed to read input file: {:?}", e))?;
        let image = image::load_from_memory(&image_buffer)
            .map_err(|_| format!("Failed to open from file descriptor: {}", self.fd))?;

        if print_info {
            let w = image.width();
            let h = image.height();
            println!(
                "w: {}, h: {}, {}",
                w, h, crate::util::make_size_string(self.size_bytes));
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
        None
    }
}

/// Wrapper for a file's location by a raw file descriptor. This owns the file descriptor now, and the file will be
/// closed when this instance is dropped.
#[derive(Debug)]
pub struct OwnedRawFdLocation {
    fd: RawFd,
    file: File,
    mime_type: String
}

impl OwnedRawFdLocation {
    pub fn new(fd: RawFd, mime_type: String) -> Self {
        let file = unsafe { File::from_raw_fd(fd) };
        Self { fd, file, mime_type }
    }
}

impl FileLocation<OwnedRawFdProperties> for OwnedRawFdLocation {

    fn is_file(&self) -> Result<bool, String> {
        let metadata = self.file.metadata()
            .map_err(|_| "Could not open metadata for file descriptor".to_owned())?;
        Ok(metadata.is_file())
    }

    fn extension(&self) -> Result<String, String> {
        let extension_index = match self.mime_type.find("/") {
            Some(i) => i + 1,
            None => return Err(format!("Could not parse MIME type: {}", self.mime_type))
        };
        if extension_index >= self.mime_type.len() - 1 {
            return Err(format!("Invalid MIME type: {}", self.mime_type));
        }
        Ok(self.mime_type[extension_index..].to_owned())
    }

    fn into_properties(self) -> Result<OwnedRawFdProperties, String> {

        // Get file size and modify date from its metadata
        let metadata = self.file
            .metadata()
            .map_err(|e| format!("Failed reading metadata: {:?}", e))?;
        let modify_time = metadata
            .modified()
            .map_err(|e| format!("Failed reading modify date: {:?}", e))?;
        let size_bytes = metadata.len();

        // All seems well, get this file's properties
        let properties = OwnedRawFdProperties {
            fd: self.fd,
            file: self.file,
            mime_type: self.mime_type,
            modify_time,
            size_bytes
        };
        Ok(properties)
    }
}
