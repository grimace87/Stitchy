pub mod image_set;
pub mod enums;

use enums::AlignmentMode;
use image_set::{FileData, ImageSet};
use std::path::{PathBuf, Path};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "")]
struct Opt {

    #[structopt(long)]
    help: bool,

    #[structopt(long)]
    version: bool,

    #[structopt(short, long)]
    horizontal: bool,

    #[structopt(short, long)]
    vertical: bool,

    #[structopt(long, default_value="0")]
    maxd: usize,

    #[structopt(long, default_value="0")]
    maxw: usize,

    #[structopt(long, default_value="0")]
    maxh: usize,

    #[structopt(short, long)]
    reverse: bool,

    #[structopt(long, default_value="100")]
    quality: usize,

    #[structopt(required_unless_one = &["help", "version"])]
    number_of_files: Option<usize>
}

fn main() {

    // Get command line args, check for flags that merely print to the console
    let mut opt: Opt = Opt::from_args();
    if opt.help {
        print_help();
        return;
    }
    if opt.version {
        print_version();
        return;
    }

    // Verify not requesting both horizontal and vertical
    if opt.horizontal && opt.vertical {
        println!("Choose either horizontal or vertical (or neither), not both.");
        return;
    }

    // Verify not requesting overlapping constraints
    if opt.maxd > 0 && opt.maxw > 0 {
        println!("If using maxd, do not specify maxw as well.");
        return;
    }
    if opt.maxd > 0 && opt.maxh > 0 {
        println!("If using maxd, do not specify maxh as well.");
        return;
    }
    if opt.maxd > 0 {
        opt.maxw = opt.maxd;
        opt.maxh = opt.maxd;
    }

    // Verify quality setting is within the appropriate range
    if opt.quality == 0 || opt.quality > 100 {
        println!("The quality setting must be in the range of 1 to 100 inclusive.");
        return;
    }

    // Verify a sensible number was given
    let number_of_files = match opt.number_of_files {
        Some(num) => num,
        _ => {
            println!("You did not provide number_of_files and StructOpt did not catch this error");
            return;
        }
    };
    if number_of_files < 2 {
        println!("You must supply a number of images that is at least 2");
        return;
    }

    // Get all accepted image files in the current directory
    let mut image_files: Vec<FileData> = match ImageSet::image_files_in_directory(vec!()) {
        Ok(files) => files,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    // Verify at least n images were found, where n is the number requested
    if image_files.len() < number_of_files {
        println!("Requested {} files, found {}", number_of_files, image_files.len());
        return;
    }

    // Sort files by modify date and take the most recent n files
    image_files.sort_unstable_by(|a, b| a.modify_time.cmp(&b.modify_time).reverse());
    image_files.truncate(number_of_files);

    // Revert to chronological order, unless the reverse order wa requested
    if !opt.reverse {
        image_files.sort_unstable_by(|a, b| a.modify_time.cmp(&b.modify_time));
    }

    // Determine alignment mode to use
    let alignment: AlignmentMode = match (opt.horizontal, opt.vertical) {
        (true, false) => AlignmentMode::Horizontal,
        (false, true) => AlignmentMode::Vertical,
        _ => AlignmentMode::Grid
    };

    // Get the next file name that can be used (stitch.jpeg, stitch_1.jpg, stitch_2.jpg, ...)
    let file_name = match next_available_image_name() {
        Ok(name) => name,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    let output_file_path = Path::new(&file_name);

    // Process the files and generate output
    match ImageSet::process_files(&output_file_path, opt.quality, image_files, alignment, opt.maxw, opt.maxh) {
        Ok(()) => println!("Created file: {}", file_name),
        Err(error) => println!("{}", error)
    }
}

fn next_available_image_name() -> Result<String, String> {

    // Get current path, check if the default file name exists, if not return it
    let mut current_path: PathBuf = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Could not access current directory"))
    };
    current_path.push("stitch.jpg");
    if !current_path.is_file() {
        return Ok(String::from("stitch.jpg"));
    }
    current_path.pop();

    // Check file names until a usable one is found
    let mut i = 1usize;
    while i < 1000 {
        let file_name: String = format!("stitch_{}.jpg", i);
        current_path.push(&file_name);
        if !current_path.is_file() {
            return Ok(file_name);
        }
        current_path.pop();
        i = i + 1;
    };
    return Err(String::from("Did not find a usable file name - if you have 1000 stitches, please move or delete some."));
}

fn print_help() {
    println!("Stitchy v{} by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    println!("Collects a number of image files in the current directory and stitches them into");
    println!("a single file.");
    println!();
    println!("Basic usage:");
    println!("  stitchy n");
    println!("  where n is the number of images to use. The most recent images available will be");
    println!("  used. There must be at least that many in the current directory.");
    println!();
    println!("Supported flags:");
    println!("  --help            Print this help");
    println!("  --version         Print the installed version number");
    println!("  --horizontal, -h  Force stitching across a single row only");
    println!("  --vertical, -v    Force stitching down a single column only");
    println!("  --maxw=n          Limit output width to n pixels at most");
    println!("  --maxh=n          Limit output height to n pixels at most");
    println!("  --maxd=n          Limit output width and height to n pixels at most");
    println!("  --reverse, -r     Stitch file in reverse chronological order");
    println!("  --quality=n       Set the output quality from 1 to 100, defaults to 100");
}

fn print_version() {
    println!("Stitchy version {}", env!("CARGO_PKG_VERSION"));
}
