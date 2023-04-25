mod file_util;
mod options;
mod print;
mod profiles;

#[cfg(test)]
mod tests;

use options::Opt;
use stitchy_core::{Stitch, ImageFiles, OrderBy, TakeFrom, util::make_size_string};
use clap::Parser;

fn main() {

    // Get command line args, check for flags that merely print to the console and exit
    let mut opt = Opt::parse();
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
    let mut previous_options: Option<Opt> = None;
    if opt.setdefaults {
        if let Some(error) = opt.check_for_basic_errors(&None) {
            println!("Cannot save settings. {}", error);
            return;
        }
        if let Some(json) = opt.serialise() {
            profiles::Profile::main().write_string(json);
        }
    } else if opt.cleardefaults {
        profiles::Profile::main().delete();
    } else if let Some(json) = profiles::Profile::main().into_string() {
        if let Some(profile_opt) = Opt::deserialise_as_current(&json) {
            opt = opt.mix_in(&profile_opt);
            previous_options = Some(profile_opt);
        }
    }

    // Check conditions where the user did not request a number of files, but this is allowed
    // because some operations on the defaults file does not require that files are processed now
    if opt.number_of_files.is_none() && (opt.setdefaults || opt.cleardefaults) {
        return;
    }

    // Perform simple validation
    if let Some(error) = opt.check_for_basic_errors(&previous_options) {
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
fn run_with_options(opt: Opt) -> Result<String, String> {

    // Determine the list of files to use as input, and from those, determine the output path
    let number_of_files = opt.number_of_files.ok_or_else(|| String::from(
        "Internal error - sorting files before verifying that a number was supplied"))?;
    let image_sources = ImageFiles::builder()
        .add_current_directory(vec![])?
        .build()?
        .sort_and_truncate_by(
            number_of_files,
            opt.order.unwrap_or(OrderBy::Latest),
            opt.take_from.unwrap_or(TakeFrom::Start),
            opt.reverse
        )?;
    let total_source_size = image_sources.total_size();
    let output_format = file_util::determine_output_format(&image_sources, &opt)?;
    let output_file_path = file_util::next_available_output(&image_sources, &opt)?;

    // Open the image files and process them to make the output image
    let images = image_sources.into_image_contents(true)?;
    let output = Stitch::builder()
        .images(images)
        .alignment(opt.get_alignment())
        .width_limit(opt.maxw as u32)
        .height_limit(opt.maxh as u32)
        .stitch()?;

    // Write the output file, returning a success message or an error message
    file_util::write_image_to_file(output, &output_file_path, output_format, opt.quality)?;
    let output_string = match file_util::size_of_file(&output_file_path) {
        Ok(size_bytes) =>
            format!(
                "Created file: {:?}, {}, ({})",
                output_file_path.file_name().unwrap(),
                make_size_string(size_bytes),
                file_util::make_ratio_string(total_source_size, size_bytes)),
        Err(_) =>
            format!(
                "Created file: {:?}",
                output_file_path.file_name().unwrap())
    };
    Ok(output_string)
}
