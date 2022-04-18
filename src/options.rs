
use serde::{Serialize, Deserialize};
use structopt::StructOpt;
use crate::{ImageFormat, AlignmentMode};

const DEFAULT_QUALITY: usize = 100;

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
#[structopt(name = "")]
pub struct Opt {

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub help: bool,

    #[structopt(long)]
    #[serde(skip_serializing, default)]
    pub version: bool,

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

    #[structopt(required_unless_one = &["help", "version"])]
    pub number_of_files: Option<usize>
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            help: false,
            version: false,
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
            number_of_files: None
        }
    }
}

impl Opt {

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

    pub fn check_for_basic_errors(&self) -> Option<&'static str> {

        // Verify not requesting both ascending and descending alphabetical order
        if self.ascalpha && self.descalpha {
            return Some("If selecting files based on alphabetical order, choose ascending or descending, not both.");
        }

        // Verify not requesting both horizontal and vertical
        if self.horizontal && self.vertical {
            return Some("Choose either horizontal or vertical (or neither), not both.");
        }

        // Verify not requesting overlapping constraints
        if self.maxd > 0 && self.maxw > 0 {
            return Some("If using maxd, do not specify maxw as well.");
        }
        if self.maxd > 0 && self.maxh > 0 {
            return Some("If using maxd, do not specify maxh as well.");
        }

        // Choose one format only, or none at all
        let format_flag_set: [bool; 4] = [self.jpeg, self.png, self.gif, self.bmp];
        let format_flag_count: i32 = format_flag_set.iter().map(|&f| { if f { 1 } else { 0 } }).sum();
        if format_flag_count > 1 {
            return Some("You cannot specify more than one of image types JPEG, PNG, GIF and BMP.");
        }

        // Verify quality setting is within the appropriate range, and is only used for JPEG
        if self.quality == 0 || self.quality > 100 {
            return Some("The quality setting must be in the range of 1 to 100 inclusive.");
        }
        if self.quality != 100 && !self.jpeg && format_flag_count > 0 {
            return Some("The quality setting can only be used for JPEG output.");
        }

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
}

#[cfg(test)]
mod test {
    use super::Opt;

    const TEST_JSON: &str = "{ \
         \"horizontal\": true, \
         \"vertical\": false, \
         \"maxd\": 0, \
         \"maxw\": 120, \
         \"maxh\": 0, \
         \"reverse\": false, \
         \"jpeg\": true, \
         \"png\": false, \
         \"gif\": false, \
         \"bmp\": false, \
         \"quality\": 80, \
         \"ascalpha\": false, \
         \"descalpha\": false, \
         \"number_of_files\": null \
         }";

    fn trim_all(s: &str) -> String {
        s.split_whitespace().collect()
    }

    fn make_test_default() -> Opt {
        Opt {
            number_of_files: Some(1),
            ..Opt::default()
        }
    }

    #[test]
    fn deserializes_okay() {
        let expected = Opt {
            horizontal: true,
            maxw: 120,
            jpeg: true,
            quality: 80,
            number_of_files: None,
            ..Opt::default()
        };
        let result = Opt::deserialise(TEST_JSON);
        assert!(result.is_some());
        let opt = result.unwrap();
        assert_eq!(format!("{:?}", expected), format!("{:?}", opt));
    }

    #[test]
    fn serializes_okay() {
        let from_opt = Opt {
            horizontal: true,
            maxw: 120,
            jpeg: true,
            quality: 80,
            number_of_files: None,
            ..Opt::default()
        };
        let result = from_opt.serialise();
        assert!(result.is_some());
        let json = result.unwrap();
        let trimmed_json = trim_all(json.as_str());
        let expected = trim_all(TEST_JSON);
        assert_eq!(expected, trimmed_json);
    }

    #[test]
    fn test_default_is_valid() {
        let error = make_test_default().check_for_basic_errors();
        assert!(error.is_none());
    }

    #[test]
    fn default_quailty_is_100() {
        let opt = make_test_default();
        assert_eq!(opt.quality, super::DEFAULT_QUALITY);
    }

    #[test]
    fn choosing_both_directions_gives_error() {
        let error = Opt { horizontal: true, vertical: true, ..make_test_default() }
            .check_for_basic_errors();
        assert!(error.is_some());
    }

    #[test]
    fn choosing_neither_direction_gives_no_error() {
        let opt = make_test_default();
        assert_eq!(opt.horizontal, false);
        assert_eq!(opt.vertical, false);
        let error = opt.check_for_basic_errors();
        assert!(error.is_none());
    }

    #[test]
    fn setting_general_and_specific_dimension_constraints_gives_error() {
        let error_1 = Opt { maxd: 100, maxw: 100, ..make_test_default() }.check_for_basic_errors();
        let error_2 = Opt { maxd: 100, maxh: 100, ..make_test_default() }.check_for_basic_errors();
        let error_3 = Opt { maxd: 100, maxw: 100, maxh: 100, ..make_test_default() }
            .check_for_basic_errors();
        assert!(error_1.is_some());
        assert!(error_2.is_some());
        assert!(error_3.is_some());
    }

    #[test]
    fn setting_both_specific_dimension_constraints_gives_no_error() {
        let error = Opt { maxw: 100, maxh: 100, ..make_test_default() }.check_for_basic_errors();
        assert!(error.is_none());
    }

    #[test]
    fn setting_zero_dimension_constraints_gives_no_error() {
        let error = Opt { maxd: 0, maxw: 0, maxh: 0, ..make_test_default() }
            .check_for_basic_errors();
        assert!(error.is_none());
    }

    #[test]
    fn choosing_multiple_formats_gives_error() {
        let error_1 = Opt { jpeg: true, png: true, ..make_test_default() }.check_for_basic_errors();
        let error_2 = Opt { png: true, gif: true, ..make_test_default() }.check_for_basic_errors();
        let error_3 = Opt { gif: true, bmp: true, ..make_test_default() }.check_for_basic_errors();
        let error_4 = Opt { jpeg: true, png: true, gif: true, bmp: true, ..make_test_default() }
            .check_for_basic_errors();
        assert!(error_1.is_some());
        assert!(error_2.is_some());
        assert!(error_3.is_some());
        assert!(error_4.is_some());
    }

    #[test]
    fn choosing_no_format_gives_no_error() {
        let opt = Opt { ..make_test_default() };
        assert_eq!(opt.jpeg, false);
        assert_eq!(opt.png, false);
        assert_eq!(opt.gif, false);
        assert_eq!(opt.bmp, false);
        let error = opt.check_for_basic_errors();
        assert!(error.is_none());
    }

    #[test]
    fn choosing_quality_for_non_jpeg_gives_error() {
        let error_1 = Opt { png: true, quality: 50, ..make_test_default() }
            .check_for_basic_errors();
        let error_2 = Opt { gif: true, quality: 50, ..make_test_default() }
            .check_for_basic_errors();
        let error_3 = Opt { bmp: true, quality: 50, ..make_test_default() }
            .check_for_basic_errors();
        assert!(error_1.is_some());
        assert!(error_2.is_some());
        assert!(error_3.is_some());
    }

    #[test]
    fn choosing_quality_for_jpeg_gives_no_error() {
        let error = Opt { jpeg: true, quality: 50, ..make_test_default() }.check_for_basic_errors();
        assert!(error.is_none());
    }

    #[test]
    fn choosing_silly_quality_gives_error() {
        let error = Opt { jpeg: true, quality: 250, ..make_test_default() }.check_for_basic_errors();
        assert!(error.is_some());
    }

    #[test]
    fn choosing_ascending_and_descending_gives_error() {
        let error = Opt { ascalpha: true, descalpha: true, ..make_test_default() }
            .check_for_basic_errors();
        assert!(error.is_some());
    }

    #[test]
    fn choosing_neither_ascending_nor_descending_gives_no_error() {
        let opt = make_test_default();
        assert_eq!(opt.ascalpha, false);
        assert_eq!(opt.descalpha, false);
        let error = opt.check_for_basic_errors();
        assert!(error.is_none());
    }
}
