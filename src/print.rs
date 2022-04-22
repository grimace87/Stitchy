
pub fn help() {
    println!("Stitchy v{} by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    println!("Collects a number of image files in the current directory and stitches them into");
    println!("a single file.");
    println!();
    println!("Basic usage:");
    println!("  stitchy n");
    println!("  where n is the number of images to use. There must be at least that many in the");
    println!("  current directory. By default, The most recent images available will be used.");
    println!();
    println!("Supported flags:");
    println!("  --help            Print this help");
    println!("  --version         Print the installed version number");
    println!("  --setdefaults     Store the given options as defaults for the current user");
    println!("  --cleardefaults   Delete the default options for the current user");
    println!("  --printdefaults   Print the default options for the current user");
    println!("  --ascalpha        Select first files based on ascending alphabetical order");
    println!("  --descalpha       Select first files based on descending alphabetical order");
    println!("  --horizontal, -h  Force stitching across a single row only");
    println!("  --vertical, -v    Force stitching down a single column only");
    println!("  --maxw=n          Limit output width to n pixels at most");
    println!("  --maxh=n          Limit output height to n pixels at most");
    println!("  --maxd=n          Limit output width and height to n pixels at most");
    println!("  --reverse, -r     Stitch file in reverse chronological order");
    println!("  --quality=n       Set the output quality from 1 to 100, defaults to 100");
    println!("  --jpeg            Output as JPEG");
    println!("  --png             Output as PNG");
    println!("  --gif             Output as GIF");
    println!("  --bmp             Output as BMP");
    println!("                    Note: default format matches sources, or JPEG if source formats vary");
    println!();
    println!("User defaults:");
    println!("  Default options can be set for the current user by using the --setdefaults flag,");
    println!("  queried using the --printdefaults flag, and deleted with --cleardefaults.");
    println!(
        "  These are stored in {} in the home directory. The next time you use Stitchy,",
        crate::profiles::PROFILE_FILE_NAME);
    println!("  defaults will be automatically applied, though can be overridden with the same flag.");
    println!("  or another flag which would perform a similar action (such as a different output format.");
    println!("  When setting defaults again, the existing ones are effectively cleared beforehand.");
}

pub fn version() {
    println!("Stitchy version {}", env!("CARGO_PKG_VERSION"));
    println!("Authored by {}", env!("CARGO_PKG_AUTHORS"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
}

pub fn defaults() {
    let load_attempt = crate::profiles::Profile::main().to_string();
    if load_attempt.is_none() {
        println!("Did not find a {} for the current user.", crate::profiles::PROFILE_FILE_NAME);
        return;
    }

    let json = load_attempt.unwrap();
    let serialise_result = crate::options::Opt::deserialise(&json);
    if serialise_result.is_none() {
        println!("Failed to parse {} for the current user.", crate::profiles::PROFILE_FILE_NAME);
        return;
    }

    // Print JSON stored in the file
    // Deserialisation was validated above since running Stitchy will do this also
    println!("{}", json);
}
