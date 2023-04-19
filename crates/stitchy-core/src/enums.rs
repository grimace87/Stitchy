
use clap::ValueEnum;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Copy, Clone, Default, ValueEnum, Serialize, Deserialize)]
pub enum TakeFrom {
    #[default]
    Start,
    End
}

#[derive(PartialEq, Debug, Copy, Clone, Default, ValueEnum, Serialize, Deserialize)]
pub enum OrderBy {
    #[default]
    Latest,
    Alphabetic
}

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
