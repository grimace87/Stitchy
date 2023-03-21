
use serde::Deserialize;
use structopt::StructOpt;

const DEFAULT_QUALITY: usize = 100;

#[derive(Debug, Clone, StructOpt, Deserialize)]
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
