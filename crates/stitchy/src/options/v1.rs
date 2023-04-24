
use clap::Parser;
use serde::Deserialize;

const DEFAULT_QUALITY: usize = 100;

#[derive(Debug, Clone, Parser, Deserialize)]
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

    #[arg(long)]
    pub jpeg: bool,

    #[arg(long)]
    pub png: bool,

    #[arg(long)]
    pub gif: bool,

    #[arg(long)]
    pub bmp: bool,

    #[arg(long, default_value="100")]
    pub quality: usize,

    #[arg(long)]
    pub ascalpha: bool,

    #[arg(long)]
    pub descalpha: bool,

    #[arg(required_unless_present_any =
    &["help", "version", "setdefaults", "cleardefaults", "printdefaults"])]
    pub number_of_files: Option<usize>,

    #[arg(long)]
    #[serde(skip_serializing, default)]
    pub setdefaults: bool,

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
}
