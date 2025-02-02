mod file_util;
mod options;
mod print;
mod profiles;

#[cfg(test)]
mod tests;

use clap::Parser;
use options::Opt;
use stitchy_core::{
    image::FilterType, util::make_size_string, FilePathWithMetadata, ImageFiles, OrderBy, Stitch,
    TakeFrom,
};

fn main() {
    // Get command line args, check for flags that merely print to the console and exit
    let opt = Opt::parse();
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

    // Modify options if requested, or try to load stored options otherwise
    let mut opt = match process_defaults_and_prepare_opt(opt) {
        Ok(None) => {
            return;
        }
        Ok(Some(opt)) => opt,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    // Check conditions where the user did not request a number of files, but this is allowed
    // because some operations on the defaults file does not require that files are processed now
    if opt.number_of_files.is_none() && (opt.setdefaults || opt.cleardefaults || opt.updatedefaults)
    {
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
        Err(msg) => println!("{}", msg),
    }
}

/// Runs Stitchy using the supplied options. The options should have been checked for basic errors
/// and prepared for use before calling this function.
fn run_with_options(opt: Opt) -> Result<String, String> {
    // Determine the list of files to use as input
    let number_of_files = opt.number_of_files.ok_or_else(|| {
        String::from("Internal error - sorting files before verifying that a number was supplied")
    })?;
    let unsorted_sources = match &opt.input_dir {
        Some(source_path) => {
            let dir = file_util::to_absolute_dir(source_path)?;
            ImageFiles::<FilePathWithMetadata>::builder()
                .add_directory(dir)?
                .build()?
        }
        None => ImageFiles::<FilePathWithMetadata>::builder()
            .add_current_directory(vec![])?
            .build()?,
    };
    let image_sources = unsorted_sources.sort_and_truncate_by(
        number_of_files,
        opt.order.unwrap_or(OrderBy::Latest),
        opt.take_from.unwrap_or(TakeFrom::Start),
        opt.reverse,
    )?;

    // Determine the output path, considering the input files if need be
    let total_source_size = image_sources.total_size();
    let output_format = file_util::determine_output_format(&image_sources, &opt)?;
    let output_file_path = file_util::next_available_output(&image_sources, &opt)?;

    // Open the image files and process them to make the output image
    let images = image_sources.into_image_contents(true)?;
    let resize_mode = match opt.fast {
        true => FilterType::Nearest,
        false => FilterType::Lanczos3,
    };
    let output = Stitch::builder()
        .images(images)
        .alignment(opt.get_alignment())
        .width_limit(opt.maxw as u32)
        .height_limit(opt.maxh as u32)
        .resize_filter(resize_mode)
        .stitch()?;

    // Write the output file, returning a success message or an error message
    file_util::write_image_to_file(output, &output_file_path, Some(output_format), opt.quality, opt.small)?;
    let output_string = match file_util::size_of_file(&output_file_path) {
        Ok(size_bytes) => format!(
            "Created file: {:?}, {}, ({})",
            output_file_path.file_name().unwrap(),
            make_size_string(size_bytes),
            file_util::make_ratio_string(total_source_size, size_bytes)
        ),
        Err(_) => format!("Created file: {:?}", output_file_path.file_name().unwrap()),
    };
    Ok(output_string)
}

/// Checks for flags setdefaults, updatedefaults, and cleardefaults, and handles
/// those. Returns any errors encountered or an Opt to proceed with afterwards.
///
/// None returned suggests to return now without proceeding with a stitch
/// operation, which could happen if modifying settings without a number of images
/// being provided.
fn process_defaults_and_prepare_opt(provided_opt: Opt) -> Result<Option<Opt>, String> {
    let mut opt = provided_opt;
    let mut previous_options: Option<Opt> = None;

    if opt.setdefaults {
        if let Some(error) = opt.check_for_basic_errors(&None) {
            return Err(format!("Cannot save settings. {}", error));
        }
        return match opt.serialise() {
            Some(json) => {
                profiles::Profile::main().write_string(json);
                return match opt.number_of_files.is_some() {
                    true => Ok(Some(opt)),
                    false => Ok(None),
                };
            }
            None => Err("Settings could not be serialised.".to_owned()),
        };
    }

    if opt.updatedefaults {
        if let Some(error) = opt.check_for_basic_errors(&None) {
            return Err(format!("Cannot update settings. {}", error));
        }
        let Some(json) = profiles::Profile::main().into_string() else {
            return Err("Existing settings could not be found.".to_owned());
        };
        let Some(previous) = Opt::deserialise_as_current(&json) else {
            return Err("Previous settings could not be successfully read.".to_owned());
        };
        opt = opt.mix_in(&previous);
        if let Some(error) = opt.check_for_basic_errors(&None) {
            return Err(error);
        }
        if let Some(json) = opt.serialise() {
            profiles::Profile::main().write_string(json);
        }
        return match opt.number_of_files.is_some() {
            true => Ok(Some(opt)),
            false => Ok(None),
        };
    }

    if opt.cleardefaults {
        profiles::Profile::main().delete();
        return match opt.number_of_files.is_some() {
            true => Ok(Some(opt)),
            false => Ok(None),
        };
    }

    if let Some(json) = profiles::Profile::main().into_string() {
        if let Some(profile_opt) = Opt::deserialise_as_current(&json) {
            opt = opt.mix_in(&profile_opt);
            previous_options = Some(profile_opt);
        }
    }

    if let Some(error) = opt.check_for_basic_errors(&previous_options) {
        return Err(error);
    }

    Ok(Some(opt))
}
