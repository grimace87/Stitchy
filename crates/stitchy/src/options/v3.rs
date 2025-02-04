use crate::options::{OptV1, OptV2};
use clap::Parser;
use serde::{Deserialize, Serialize};
use stitchy_core::{OrderBy, TakeFrom};

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

    #[arg(long, default_value = "0")]
    pub maxd: usize,

    #[arg(long, default_value = "0")]
    pub maxw: usize,

    #[arg(long, default_value = "0")]
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

    #[arg(long, default_value = "100")]
    pub quality: usize,

    #[arg(long)]
    pub order: Option<OrderBy>,

    #[arg(short, long = "input-dir")]
    pub input_dir: Option<String>,

    #[arg(short, long = "output-dir")]
    pub output_dir: Option<String>,

    #[arg(required_unless_present_any =
    &["help", "version", "setdefaults", "cleardefaults", "printdefaults"])]
    pub number_of_files: Option<usize>,

    #[arg(long)]
    #[serde(skip_serializing, default)]
    pub setdefaults: bool,

    #[arg(long)]
    #[serde(skip_serializing, default)]
    pub cleardefaults: bool,
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
            quality: DEFAULT_QUALITY,
            order: None,
            input_dir: None,
            output_dir: None,
            number_of_files: None,
            setdefaults: false,
            cleardefaults: false,
        }
    }
}

impl Opt {
    pub fn deserialise(json: &str) -> Result<Opt, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("Error deserialising settings: {:?}", e))
    }
}

impl From<OptV2> for Opt {
    fn from(value: OptV2) -> Self {
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
            quality: value.quality,
            order: value.order,
            input_dir: None,
            output_dir: None,
            number_of_files: value.number_of_files,
            setdefaults: value.setdefaults,
            cleardefaults: value.cleardefaults,
        }
    }
}

impl From<OptV1> for Opt {
    fn from(value: OptV1) -> Self {
        let value: OptV2 = value.into();
        value.into()
    }
}
