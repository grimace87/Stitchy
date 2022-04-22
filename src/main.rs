pub mod enums;
pub mod files;
pub mod image_set;
pub mod options;
pub mod print;
pub mod profiles;

use enums::{AlignmentMode, ImageFormat};
use files::ImageFiles;
use image_set::ImageSet;
use structopt::StructOpt;

fn main() {

    // Get command line args, check for flags that merely print to the console and exit
    let mut opt = options::Opt::from_args();
    if opt.help {
        print::help();
        return;
    }
    if opt.version {
        print::version();
        return;
    }
    if opt.printdefaults {
        print::defaults();
        return;
    }

    // Save options if requested, or try to load stored options otherwise
    if opt.setdefaults {
        if let Some(error) = opt.check_for_basic_errors() {
            println!("Cannot save settings. {}", error);
            return;
        }
        if let Some(json) = opt.serialise() {
            profiles::Profile::main().write_string(json);
        }
    } else if opt.cleardefaults {
        profiles::Profile::main().delete();
    } else if let Some(json) = profiles::Profile::main().to_string() {
        if let Some(profile_opt) = options::Opt::deserialise(&json) {
            opt = opt.mix_in(profile_opt);
        }
    }

    // Check conditions where the user did not request a number of files, but this is allowed
    // because some operations on the defaults file does not require that files are processed now
    if opt.number_of_files.is_none() && (opt.setdefaults || opt.cleardefaults) {
        return;
    }

    // Perform simple validation
    if let Some(error) = opt.check_for_basic_errors() {
        println!("{}", error);
        return;
    }

    // Ensure some number of files was provided
    if let Some(error) = opt.check_number_of_files_provided() {
        println!("{}", error);
        return;
    }

    // Pre-use preparations
    opt.prepare_for_use();

    // Call function to do all the file processing, print final messages here
    match run_with_options(opt) {
        Ok(msg) => println!("{}", msg),
        Err(msg) => println!("{}", msg)
    }
}

/// Runs Stitchy using the supplied options. The options should have been checked for basic errors
/// and prepared for use before calling this function.
fn run_with_options(opt: options::Opt) -> Result<String, String> {

    // Determine the list of files to use as input, and from those, determine the output path
    let image_sources = ImageFiles::from_directory(vec!())?
        .sort_and_truncate_by(&opt)?;
    let output_format = image_sources.determine_output_format(&opt)?;
    let output_file_path = image_sources.next_available_output(&opt)?;

    // Open the image files and process them to make the output image
    let images = image_sources.into_image_contents()?;
    let output = ImageSet::new(images, &opt)
        .stitch()?;

    // Write the output file, returning a success message or an error message
    files::write_image_to_file(output, &output_file_path, output_format, opt.quality)?;
    let output_string = match files::size_of_file(&output_file_path) {
        Ok(size_string) => format!(
            "Created file: {:?}, {}", output_file_path.file_name().unwrap(), size_string
        ),
        Err(_) => format!(
            "Created file: {:?}", output_file_path.file_name().unwrap()
        )
    };
    Ok(output_string)
}
