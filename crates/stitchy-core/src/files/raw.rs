use crate::{FileLocation, FileProperties};
use image::{metadata::Orientation, DynamicImage, ImageFormat};
use std::io::Cursor;
use std::time::SystemTime;

/// Wrapper for a byte buffer, paired with data that cannot be extracted easily.
#[derive(Debug)]
pub struct RawBufferProperties<'a> {
    buffer: &'a [u8],
    mime_type: String,
    modify_time: SystemTime,
}

impl<'a> RawBufferProperties<'a> {
    fn new(
        buffer: &'a [u8],
        mime_type: String,
        modify_time: SystemTime
    ) -> Result<Self, String> {
        Ok(Self { buffer, mime_type, modify_time })
    }
}

impl<'a> FileProperties for RawBufferProperties<'a> {
    fn infer_format(&self) -> Option<ImageFormat> {
        match self.mime_type.as_str() {
            "image/jpeg" => Some(ImageFormat::Jpeg),
            "image/png" => Some(ImageFormat::Png),
            "image/gif" => Some(ImageFormat::Gif),
            "image/bmp" => Some(ImageFormat::Bmp),
            "image/webp" => Some(ImageFormat::WebP),
            _ => None,
        }
    }

    fn into_image_contents(self, print_info: bool) -> Result<DynamicImage, String> {
        let image = image::load_from_memory(self.buffer)
            .map_err(|e| format!("Failed to open file buffer: {:?}", e))?;

        if print_info {
            let w = image.width();
            let h = image.height();
            println!(
                "w: {}, h: {}, {}",
                w,
                h,
                crate::util::make_size_string(self.buffer.len() as u64)
            );
        }

        Ok(image)
    }

    #[inline]
    fn file_size(&self) -> u64 {
        self.buffer.len() as u64
    }

    #[inline]
    fn modify_time(&self) -> SystemTime {
        self.modify_time
    }

    #[inline]
    fn full_path(&self) -> Option<&String> {
        None
    }

    #[inline]
    fn orientation(&self) -> Result<Orientation, String> {
        let reader = Cursor::new(self.buffer);
        self.decode_orientation(reader)
    }
}

/// Wrapper for a file's raw data, and everything that cannot be obtained from that data.
#[derive(Debug)]
pub struct RawBufferLocation<'a> {
    buffer: &'a [u8],
    mime_type: String,
    modify_time: SystemTime
}

impl<'a> RawBufferLocation<'a> {
    pub fn new(
        buffer: &'a [u8],
        mime_type: String,
        modify_time: SystemTime
    ) -> Self {
        Self {
            buffer,
            mime_type,
            modify_time
        }
    }
}

impl<'a> FileLocation<RawBufferProperties<'a>> for RawBufferLocation<'a> {
    fn is_file(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn extension(&self) -> Result<String, String> {
        let extension_index = match self.mime_type.find("/") {
            Some(i) => i + 1,
            None => return Err(format!("Could not parse MIME type: {}", self.mime_type)),
        };
        if extension_index >= self.mime_type.len() - 1 {
            return Err(format!("Invalid MIME type: {}", self.mime_type));
        }
        Ok(self.mime_type[extension_index..].to_owned())
    }

    fn into_properties(self) -> Result<RawBufferProperties<'a>, String> {
        RawBufferProperties::new(
            self.buffer,
            self.mime_type,
            self.modify_time
        )
    }
}
