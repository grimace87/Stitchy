
use crate::image::ImageOutputFormat;

#[cfg(feature = "parser")]
use clap::ValueEnum;

#[cfg(feature = "parser")]
use serde::{Serialize, Deserialize};

/// Configure which end of the set of files to take from. The first file used will be the one
/// at the specified end, and then the next file in from the end, and so on.
/// The meaning of [TakeFrom::Start] or [TakeFrom::End] depends on the ordering specified with
/// [OrderBy].
///
/// With files named "a.jpg", "b.jpg", and "c.jpg", electing to stitch 2 files together with
/// [OrderBy::Alphabetic], then [TakeFrom::Start] will process files "a.jpg" then "b.jpg",
/// while [TakeFrom::End] will process files "c.jpg" then "b.jpg".
///
/// For ordering [OrderBy::Latest], the last-updated timestamps of the files determines the
/// order: [TakeFrom::Start] will begin with the most recent file first and working backwards,
/// while [TakeFrom::End] will take the oldest file and work forwards.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "parser", derive(ValueEnum, Serialize, Deserialize))]
pub enum TakeFrom {
    #[default]
    Start,
    End
}

/// Configure the order in which files are taken when selecting files from a set.
/// Specify which end of the list to take files from when stitching using [TakeFrom].
#[derive(PartialEq, Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "parser", derive(ValueEnum, Serialize, Deserialize))]
pub enum OrderBy {
    #[default]
    Latest,
    Alphabetic
}

/// Enum of supported image formats.
/// Includes conversion function to [image::ImageOutputFormat], though this requires the
/// quality setting that will only be used for JPEG images.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum ImageFormat {
    #[default]
    Unspecified,
    Jpeg,
    Png,
    Gif,
    Bmp
}

impl ImageFormat {

    pub fn allowed_extensions() -> [&'static str; 5] {
        ["jpg", "jpeg", "png", "gif", "bmp"]
    }

    pub fn infer_format(file_name: &str) -> ImageFormat {
        match file_name.rfind('.') {
            Some(pos) => {
                let extension = &file_name[pos..file_name.len()];
                for &(ext, fmt) in Self::extension_formats().iter() {
                    if ext == extension {
                        return fmt;
                    }
                }
                ImageFormat::Unspecified
            },
            None => ImageFormat::Unspecified
        }
    }

    pub fn get_main_extension(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Png => "png",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Unspecified => "jpg"
        }
    }

    pub fn to_image_output_format(self, quality: usize) -> ImageOutputFormat {
        match self {
            ImageFormat::Jpeg => ImageOutputFormat::Jpeg(quality as u8),
            ImageFormat::Png => ImageOutputFormat::Png,
            ImageFormat::Gif => ImageOutputFormat::Gif,
            ImageFormat::Bmp => ImageOutputFormat::Bmp,
            ImageFormat::Unspecified => ImageOutputFormat::Jpeg(100u8)
        }
    }

    fn extension_formats() -> [(&'static str, ImageFormat); 5] {
        [
            (".jpg", ImageFormat::Jpeg),
            (".jpeg", ImageFormat::Jpeg),
            (".png", ImageFormat::Png),
            (".gif", ImageFormat::Gif),
            (".bmp", ImageFormat::Bmp)
        ]
    }
}
