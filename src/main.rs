pub mod image_set;
pub mod enums;

use enums::AlignmentMode;
use image_set::{FileData, ImageSet};
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

    // Determine alignment mode to use
    let alignment: AlignmentMode = match (opt.horizontal, opt.vertical) {
        (true, false) => AlignmentMode::Horizontal,
        (false, true) => AlignmentMode::Vertical,
        _ => AlignmentMode::Grid
    };

    // Process the files and generate output
    let file_name = "./stitch.jpg";
    match ImageSet::process_files(file_name, image_files, alignment, opt.maxw, opt.maxh) {
        Ok(()) => println!("Created file: {}", file_name),
        Err(error) => println!("{}", error)
    }
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
}

fn print_version() {
    println!("Stitchy version {}", env!("CARGO_PKG_VERSION"));
}
