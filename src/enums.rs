
pub enum AspectType {
    Wide,
    Portrait,
    Squarish
}

impl AspectType {
    pub fn get_aspect_from_dims(w: u32, h: u32) -> AspectType {
        let aspect_ratio: f32 = (w as f32) / (h as f32);
        if aspect_ratio > 1.25f32 {
            AspectType::Wide
        } else if aspect_ratio < 0.8f32 {
            AspectType::Portrait
        } else {
            AspectType::Squarish
        }
    }
}

#[derive(PartialEq)]
pub enum AlignmentMode {
    Grid,
    Horizontal,
    Vertical
}

#[derive(PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical
}

#[derive(PartialEq, Copy, Clone)]
pub enum ImageFormat {
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

    pub fn infer_format(file_name: &String) -> ImageFormat {
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
