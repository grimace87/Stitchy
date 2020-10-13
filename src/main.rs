pub mod image_set;
pub mod enums;

use enums::{AlignmentMode, ImageFormat};
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

    #[structopt(long)]
    jpeg: bool,

    #[structopt(long)]
    png: bool,

    #[structopt(long)]
    gif: bool,

    #[structopt(long)]
    bmp: bool,

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

    // Choose one format only, or none at all
    let flag_set: [bool; 4] = [opt.jpeg, opt.png, opt.gif, opt.bmp];
    let format_flag_count: i32 = flag_set.iter().map(|&f| { if f { 1 } else { 0 } }).sum();
    if format_flag_count > 1 {
        println!("You cannot specify more than one of image types JPEG, PNG, GIF and BMP.");
        return;
    }
    let mut image_format: ImageFormat = if opt.jpeg {
        ImageFormat::Jpeg
    } else if opt.png {
        ImageFormat::Png
    } else if opt.gif {
        ImageFormat::Gif
    } else if opt.bmp {
        ImageFormat::Bmp
    } else {
        ImageFormat::Unspecified
    };

    // Verify quality setting is within the appropriate range, and is only used for JPEG
    if opt.quality == 0 || opt.quality > 100 {
        println!("The quality setting must be in the range of 1 to 100 inclusive.");
        return;
    }
    if opt.quality != 100 && image_format != ImageFormat::Jpeg && image_format != ImageFormat::Unspecified {
        println!("The quality setting can only be used for JPEG output.");
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
    if number_of_files == 0 {
        println!("The number of images to stitch must be at least 1.");
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

    // Check if no format was specified, but all source images are the same
    if image_format == ImageFormat::Unspecified {
        image_format = common_format_in(&image_files);
        if image_format == ImageFormat::Unspecified {
            image_format = ImageFormat::Jpeg;
        }
        if image_format != ImageFormat::Jpeg && opt.quality != 100 {
            println!("Output file with extension {} cannot use a quality setting.", image_format.get_main_extension());
            return;
        }
    }

    // Get the next file name that can be used (stitch.jpeg, stitch_1.jpg, stitch_2.jpg, ...)
    let file_name = match next_available_image_name(&image_format) {
        Ok(name) => name,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    let output_file_path = Path::new(&file_name);

    // Process the files and generate output
    match ImageSet::process_files(&output_file_path, image_format, opt.quality, image_files, alignment, opt.maxw, opt.maxh) {
        Ok(()) => println!("Created file: {}", file_name),
        Err(error) => println!("{}", error)
    }
}

fn common_format_in(image_files: &Vec<FileData>) -> ImageFormat {
    if image_files.is_empty() {
        return ImageFormat::Unspecified;
    }
    let mut all_formats = image_files.iter().map(|file_data| {
        ImageFormat::infer_format(&file_data.full_path)
    });
    let first_format = all_formats.next().unwrap();
    match all_formats.all(|fmt| fmt == first_format) {
        true => first_format,
        false => ImageFormat::Unspecified
    }
}

fn next_available_image_name(image_format: &ImageFormat) -> Result<String, String> {

    let target_extension = image_format.get_main_extension();

    // Get current path, check if the default file name exists, if not return it
    let mut current_path: PathBuf = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(String::from("Could not access current directory"))
    };
    let mut un_numbered_file_exists = false;
    for &extension in ImageFormat::allowed_extensions().iter() {
        current_path.push(format!("stitch.{}", extension));
        if current_path.is_file() {
            un_numbered_file_exists = true;
            break;
        }
        current_path.pop();
    }
    if !un_numbered_file_exists {
        return Ok(format!("stitch{}", target_extension));
    }

    // Check file names until a usable one is found
    let mut i = 1usize;
    while i < 1000 {
        let mut numbered_file_exists = false;
        for &extension in ImageFormat::allowed_extensions().iter() {
            let file_name: String = format!("stitch_{}.{}", i, extension);
            current_path.push(file_name);
            if current_path.is_file() {
                numbered_file_exists = true;
            }
            current_path.pop();
            if numbered_file_exists {
                break;
            }
        }
        if !numbered_file_exists {
            return Ok(format!("stitch_{}{}", i, target_extension));
        }
        i += 1;
    };
    Err(String::from("Did not find a usable file name - if you have 1000 stitches, please move or delete some."))
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
