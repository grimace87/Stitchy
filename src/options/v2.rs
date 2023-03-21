
use serde::{Serialize, Deserialize};
use structopt::StructOpt;
use crate::{ImageFormat, AlignmentMode};
use crate::options::OptV1;

pub const DEFAULT_QUALITY: usize = 100;

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
#[structopt(name = "")]
pub struct Opt {

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub help: bool,

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub version: bool,

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub printdefaults: bool,

    #[structopt(short, long)]
    pub horizontal: bool,

    #[structopt(short, long)]
    pub vertical: bool,

    #[structopt(long, default_value="0")]
    pub maxd: usize,

    #[structopt(long, default_value="0")]
    pub maxw: usize,

    #[structopt(long, default_value="0")]
    pub maxh: usize,

    #[structopt(short, long)]
    pub reverse: bool,

    #[structopt(long)]
    pub jpeg: bool,

    #[structopt(long)]
    pub png: bool,

    #[structopt(long)]
    pub gif: bool,

    #[structopt(long)]
    pub bmp: bool,

    #[structopt(long, default_value="100")]
    pub quality: usize,

    #[structopt(long)]
    pub ascalpha: bool,

    #[structopt(long)]
    pub descalpha: bool,

    #[structopt(required_unless_one =
    &["help", "version", "setdefaults", "cleardefaults", "printdefaults"])]
    pub number_of_files: Option<usize>,

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub setdefaults: bool,

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub cleardefaults: bool
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            help: false,
            version: false,
            printdefaults: false,
            horizontal: false,
            vertical: false,
            maxd: 0,
            maxw: 0,
            maxh: 0,
            reverse: false,
            jpeg: false,
            png: false,
            gif: false,
            bmp: false,
            quality: DEFAULT_QUALITY,
            ascalpha: false,
            descalpha: false,
            number_of_files: None,
            setdefaults: false,
            cleardefaults: false
        }
    }
}

impl Opt {

    #[inline]
    fn supports_quality(&self) -> bool {
        self.jpeg
    }

    #[inline]
    fn names_of_format_supporting_quality() -> [&'static str; 1] {
        ["JPEG"]
    }

    pub(crate) fn deserialise(json: &str) -> Option<Opt> {
        let result = serde_json::from_str(json);
        match result {
            Ok(o) => o,
            Err(e) => {
                println!("Error deserialising settings: {:?}", e);
                None
            }
        }
    }

    pub fn check_for_basic_errors(&self, previous_options: &Option<Opt>) -> Option<String> {

        // Verify not requesting both ascending and descending alphabetical order
        if self.ascalpha && self.descalpha {
            return Some("If selecting files based on alphabetical order, choose ascending or descending, not both.".to_owned());
        }

        // Verify not requesting both horizontal and vertical
        if self.horizontal && self.vertical {
            return Some("Choose either horizontal or vertical (or neither), not both.".to_owned());
        }

        // Verify not requesting overlapping constraints
        if self.maxd > 0 && self.maxw > 0 {
            return Some("If using maxd, do not specify maxw as well.".to_owned());
        }
        if self.maxd > 0 && self.maxh > 0 {
            return Some("If using maxd, do not specify maxh as well.".to_owned());
        }

        // Choose one format only, or none at all
        let format_flag_set: [bool; 4] = [self.jpeg, self.png, self.gif, self.bmp];
        let format_flag_count: usize = format_flag_set.iter()
            .map(|&f| { if f { 1 } else { 0 } })
            .sum();
        if format_flag_count > 1 {
            return Some("You cannot specify more than one of image types JPEG, PNG, GIF and BMP.".to_owned());
        }

        // Verify quality setting is within the appropriate range, and is only used for JPEG.
        // Be careful that a quality setting loaded from settings is ignored when changing format.
        if self.quality == 0 || self.quality > 100 {
            return Some("The quality setting must be in the range of 1 to 100 inclusive.".to_owned());
        }
        let quality_types = Self::names_of_format_supporting_quality();
        let targeting_quality = self.quality != 100 && format_flag_count > 0;
        let defaults_support_quality = match previous_options {
            Some(options) => options.supports_quality(),
            _ => false
        };
        if targeting_quality && !self.supports_quality() && !defaults_support_quality {
            return Some(
                format!("The quality setting can only be used for {} output.", quality_types[0]));
        }

        None
    }

    pub fn check_number_of_files_provided(&self) -> Option<&'static str> {

        // Verify a sensible number was given
        let number_of_files = match self.number_of_files {
            Some(num) => num,
            _ => return Some("You did not provide number_of_files and StructOpt did not catch this error")
        };
        if number_of_files == 0 {
            return Some("The number of images to stitch must be at least 1.");
        }

        None
    }

    pub fn prepare_for_use(&mut self) {
        if self.maxd > 0 {
            self.maxw = self.maxd;
            self.maxh = self.maxd;
        }
    }

    pub fn get_requested_image_format(&self) -> ImageFormat {
        if self.jpeg {
            ImageFormat::Jpeg
        } else if self.png {
            ImageFormat::Png
        } else if self.gif {
            ImageFormat::Gif
        } else if self.bmp {
            ImageFormat::Bmp
        } else {
            ImageFormat::Unspecified
        }
    }

    pub fn get_alignment(&self) -> AlignmentMode {
        match (self.horizontal, self.vertical) {
            (true, false) => AlignmentMode::Horizontal,
            (false, true) => AlignmentMode::Vertical,
            _ => AlignmentMode::Grid
        }
    }

    pub fn serialise(&self) -> Option<String> {
        let result = serde_json::to_string(self);
        match result {
            Ok(s) => Some(s),
            Err(e) => {
                println!("Error serialising settings: {:?}", e);
                None
            }
        }
    }

    /// Sets the options included in the other instance
    pub fn mix_in(self, other: &Opt) -> Opt {
        let number_of_files = match (self.number_of_files, other.number_of_files) {
            (Some(i), None) => Some(i),
            (None, Some(i)) => Some(i),
            (Some(i), Some(_)) => Some(i),
            _ => None
        };
        let base_has_axis = self.horizontal || self.vertical;
        let base_has_format = self.jpeg || self.png || self.gif || self.bmp;
        let base_constrains_dimensions = self.maxd != 0 || self.maxw != 0 || self.maxh != 0;
        let base_sorts_alpha = self.ascalpha || self.descalpha;
        Opt {
            help: self.help,
            version: self.version,
            printdefaults: self.printdefaults,
            horizontal: self.horizontal || (other.horizontal && !base_has_axis),
            vertical: self.vertical || (other.vertical && !base_has_axis),
            maxd: if base_constrains_dimensions { self.maxd } else { other.maxd },
            maxw: if base_constrains_dimensions { self.maxw } else { other.maxw },
            maxh: if base_constrains_dimensions { self.maxh } else { other.maxh },
            reverse: self.reverse || other.reverse,
            jpeg: self.jpeg || (other.jpeg && !base_has_format),
            png: self.png || (other.png && !base_has_format),
            gif: self.gif || (other.gif && !base_has_format),
            bmp: self.bmp || (other.bmp && !base_has_format),
            quality: if self.quality != DEFAULT_QUALITY { self.quality } else { other.quality },
            ascalpha: self.ascalpha || (other.ascalpha && !base_sorts_alpha),
            descalpha: self.descalpha || (other.descalpha && !base_sorts_alpha),
            number_of_files,
            setdefaults: self.setdefaults,
            cleardefaults: self.cleardefaults
        }
    }
}

impl From<OptV1> for Opt {
    fn from(value: OptV1) -> Self {
        Opt {
            help: value.help,
            version: value.version,
            printdefaults: value.printdefaults,
            horizontal: value.horizontal,
            vertical: value.vertical,
            maxd: value.maxd,
            maxw: value.maxw,
            maxh: value.maxh,
            reverse: value.reverse,
            jpeg: value.jpeg,
            png: value.png,
            gif: value.gif,
            bmp: value.bmp,
            quality: value.quality,
            ascalpha: value.ascalpha,
            descalpha: value.descalpha,
            number_of_files: value.number_of_files,
            setdefaults: value.setdefaults,
            cleardefaults: value.cleardefaults,
        }
    }
}
