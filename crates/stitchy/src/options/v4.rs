
use crate::{options::{OptV1, OptV2, OptV3}, file_util::to_absolute_dir};
use stitchy_core::{image::ImageFormat, AlignmentMode, TakeFrom, OrderBy};
use clap::Parser;
use serde::{Serialize, Deserialize};

pub const DEFAULT_QUALITY: usize = 100;

#[derive(Debug, Clone, Parser, Serialize, Deserialize)]
pub struct Opt {

    #[arg(long)]
    #[serde(skip_serializing, default)]
    pub help: bool,

    #[arg(long)]
    #[serde(skip_serializing, default)]
    pub version: bool,

    #[arg(long)]
    #[serde(skip_serializing, default)]
    pub printdefaults: bool,

    #[arg(short, long)]
    pub horizontal: bool,

    #[arg(short, long)]
    pub vertical: bool,

    #[arg(long, default_value="0")]
    pub maxd: usize,

    #[arg(long, default_value="0")]
    pub maxw: usize,

    #[arg(long, default_value="0")]
    pub maxh: usize,

    #[arg(short, long)]
    pub reverse: bool,

    #[arg(long = "take-from")]
    pub take_from: Option<TakeFrom>,

    #[arg(long)]
    pub jpeg: bool,

    #[arg(long)]
    pub png: bool,

    #[arg(long)]
    pub gif: bool,

    #[arg(long)]
    pub bmp: bool,

    #[arg(long)]
    pub webp: bool,

    #[arg(long)]
    pub fast: bool,

    #[arg(long, default_value="100")]
    pub quality: usize,

    #[arg(long)]
    pub order: Option<OrderBy>,

    #[arg(short, long = "input-dir")]
    pub input_dir: Option<String>,

    #[arg(short, long = "output-dir")]
    pub output_dir: Option<String>,

    #[arg(required_unless_present_any =
    &["help", "version", "setdefaults", "updatedefaults", "cleardefaults", "printdefaults"])]
    pub number_of_files: Option<usize>,

    #[arg(long)]
    #[serde(skip_serializing, default)]
    pub setdefaults: bool,

    #[arg(long)]
    #[serde(skip_serializing, default)]
    pub updatedefaults: bool,

    #[arg(long)]
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
            take_from: None,
            jpeg: false,
            png: false,
            gif: false,
            bmp: false,
            webp: false,
            fast: false,
            quality: DEFAULT_QUALITY,
            order: None,
            input_dir: None,
            output_dir: None,
            number_of_files: None,
            setdefaults: false,
            updatedefaults: false,
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

    pub fn deserialise(json: &str) -> Option<Opt> {
        let result = serde_json::from_str(json);
        match result {
            Ok(o) => o,
            Err(e) => {
                println!("Error deserialising settings: {:?}", e);
                None
            }
        }
    }

    pub fn deserialise_as_current(json: &str) -> Option<Opt> {

        // Try latest version
        if let Some(opt) = Opt::deserialise(json) {
            return Some(opt);
        }

        // Try v3
        if let Some(opt) = OptV3::deserialise(json) {
            return Some(opt.into());
        }

        // Try v2
        if let Some(opt) = OptV2::deserialise(json) {
            return Some(opt.into());
        }

        // Try v1
        if let Some(opt) = OptV1::deserialise(json) {
            return Some(opt.into());
        }

        None
    }

    pub fn check_for_basic_errors(&self, previous_options: &Option<Opt>) -> Option<String> {

        // Verify directories are actually directories
        if let Some(dir) = &self.input_dir {
            if let Err(e) = to_absolute_dir(dir) {
                return Some(e);
            }
        }
        if let Some(dir) = &self.output_dir {
            if let Err(e) = to_absolute_dir(dir) {
                return Some(e);
            }
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
        let format_flag_set: [bool; 5] = [self.jpeg, self.png, self.gif, self.bmp, self.webp];
        let format_flag_count: usize = format_flag_set.iter()
            .map(|&f| { if f { 1 } else { 0 } })
            .sum();
        if format_flag_count > 1 {
            return Some("You cannot specify more than one of image types JPEG, PNG, GIF, BMP, and WebP.".to_owned());
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
            None => false
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
            None => return Some("You did not provide number_of_files and StructOpt did not catch this error")
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

    pub fn get_requested_image_format(&self) -> Option<ImageFormat> {
        if self.jpeg {
            Some(ImageFormat::Jpeg)
        } else if self.png {
            Some(ImageFormat::Png)
        } else if self.gif {
            Some(ImageFormat::Gif)
        } else if self.bmp {
            Some(ImageFormat::Bmp)
        } else if self.webp {
            Some(ImageFormat::WebP)
        } else {
            None
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
        let take_from = match (self.take_from, other.take_from) {
            (None, that) => that,
            (this, _) => this
        };
        let base_has_axis = self.horizontal || self.vertical;
        let base_has_format = self.jpeg || self.png || self.gif || self.bmp || self.webp;
        let base_constrains_dimensions = self.maxd != 0 || self.maxw != 0 || self.maxh != 0;
        let order = match (self.order, other.order) {
            (None, that) => that,
            (this, _) => this
        };
        let input_dir = match (&self.input_dir, &other.input_dir) {
            (None, Some(that)) => Some(that.clone()),
            (Some(this), _) => Some(this.clone()),
            _ => None
        };
        let output_dir = match (&self.output_dir, &other.output_dir) {
            (None, Some(that)) => Some(that.clone()),
            (Some(this), _) => Some(this.clone()),
            _ => None
        };
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
            take_from,
            jpeg: self.jpeg || (other.jpeg && !base_has_format),
            png: self.png || (other.png && !base_has_format),
            gif: self.gif || (other.gif && !base_has_format),
            bmp: self.bmp || (other.bmp && !base_has_format),
            webp: self.webp || (other.webp && !base_has_format),
            fast: self.fast || other.fast,
            quality: if self.quality != DEFAULT_QUALITY { self.quality } else { other.quality },
            order,
            input_dir,
            output_dir,
            number_of_files,
            setdefaults: self.setdefaults,
            updatedefaults: self.updatedefaults,
            cleardefaults: self.cleardefaults
        }
    }
}

impl From<OptV3> for Opt {
    fn from(value: OptV3) -> Self {

        // Return new type
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
            take_from: value.take_from,
            jpeg: value.jpeg,
            png: value.png,
            gif: value.gif,
            bmp: value.bmp,
            webp: false,
            fast: false,
            quality: value.quality,
            order: value.order,
            input_dir: None,
            output_dir: None,
            number_of_files: value.number_of_files,
            setdefaults: value.setdefaults,
            updatedefaults: false,
            cleardefaults: value.cleardefaults,
        }
    }
}

impl From<OptV2> for Opt {
    fn from(value: OptV2) -> Self {
        let value: OptV3 = value.into();
        value.into()
    }
}

impl From<OptV1> for Opt {
    fn from(value: OptV1) -> Self {
        let value: OptV2 = value.into();
        value.into()
    }
}
