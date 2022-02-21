pub mod enums;
pub mod files;
pub mod image_set;
pub mod options;
pub mod print;

use enums::{AlignmentMode, ImageFormat};
use files::FileData;
use image_set::ImageSet;
use std::path::{PathBuf, Path};
use structopt::StructOpt;

fn main() {

    // Get command line args, check for flags that merely print to the console
    let mut opt = options::Opt::from_args();
    if opt.help {
        print::help();
        return;
    }
    if opt.version {
        print::version();
        return;
    }

    // Perform simple validation
    if let Some(error) = opt.check_for_basic_errors() {
        println!("{}", error);
        return;
    }

    // Pre-use preparations
    opt.prepare_for_use();
    let mut image_format: ImageFormat = opt.get_requested_image_format();
    let number_of_files = opt.number_of_files.unwrap();

    // Get all accepted image files in the current directory
    let mut image_files: Vec<FileData> = match FileData::image_files_in_directory(vec!()) {
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

    // Sort by ascending or descending alphabetical order, or by descending-order modify date, then take n.
    // Idea of 'naturally ordered' means that after taking n from start of the vector, the files will
    // be in the order in which they should be stitched, though the 'reverse' flag inverts this behaviour.
    let naturally_ordered: bool = if opt.ascalpha {
        image_files.sort_unstable_by(|a, b| a.full_path.cmp(&b.full_path));
        true
    } else if opt.descalpha {
        image_files.sort_unstable_by(|a, b| a.full_path.cmp(&b.full_path).reverse());
        true
    } else {
        image_files.sort_unstable_by(|a, b| a.modify_time.cmp(&b.modify_time).reverse());
        false
    };
    image_files.truncate(number_of_files);

    // Revert to chronological order, unless the reverse order was requested
    if opt.reverse ^ !naturally_ordered {
        image_files.reverse();
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
            println!("Output file with extension .{} cannot use a quality setting.", image_format.get_main_extension());
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
    match ImageSet::process_files(output_file_path, image_format, opt.quality, image_files, alignment, opt.maxw, opt.maxh) {
        Ok(()) => println!("Created file: {}", file_name),
        Err(error) => println!("{}", error)
    }
}

fn common_format_in(image_files: &[FileData]) -> ImageFormat {
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
            current_path.pop();
            break;
        }
        current_path.pop();
    }
    if !un_numbered_file_exists {
        return Ok(format!("stitch.{}", target_extension));
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
            return Ok(format!("stitch_{}.{}", i, target_extension));
        }
        i += 1;
    };
    Err(String::from("Did not find a usable file name - if you have 1000 stitches, please move or delete some."))
}
