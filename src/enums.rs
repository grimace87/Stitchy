
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
