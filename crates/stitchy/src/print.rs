use crate::profiles::{Profile, PROFILE_FILE_NAME};
use crate::Opt;

struct SplitPrinter {
    available_width: usize,
}

impl Default for SplitPrinter {
    fn default() -> Self {
        let available_width =
            termsize::get().map_or_else(|| std::usize::MAX, |size| size.cols as usize);
        Self { available_width }
    }
}

impl SplitPrinter {
    fn print_with_wrap_indent(&self, wrap_indent: usize, texts: &[String]) {
        for text in texts.iter() {
            self.print_line_with_wrap_indent(wrap_indent, text);
        }
    }

    fn print_line_with_wrap_indent(&self, wrap_indent: usize, text: &str) {
        let print_width = text.chars().count();
        if print_width <= self.available_width {
            println!("{}", text);
            return;
        }

        let indentation_is_sensible = self.available_width > wrap_indent + 8;
        let wrap_indentation = match indentation_is_sensible {
            true => " ".repeat(wrap_indent),
            false => "".to_owned(),
        };
        let print_width = self.available_width - wrap_indent;

        let length_of_first_line = text[0..self.available_width]
            .rfind(|c| c == ' ')
            .unwrap_or_else(|| self.available_width);
        let (first_line, remaining) = text.split_at(length_of_first_line);
        println!("{}", first_line);

        let mut current_line = String::new();
        current_line.push_str(&wrap_indentation);
        for word in remaining.split_whitespace() {
            let after_width = current_line.chars().count() + word.chars().count();
            if after_width <= print_width {
                current_line.push_str(word);
                if after_width + 1 <= print_width {
                    current_line.push(' ');
                }
            } else {
                println!("{}", current_line);
                current_line.clear();
                current_line.push_str(&wrap_indentation);
                current_line.push_str(word);
                current_line.push(' ');
            }
        }
        if !current_line.is_empty() {
            println!("{}", current_line);
        }
    }
}

pub fn help() {
    let printer = SplitPrinter::default();
    printer.print_with_wrap_indent(0, &[
        format!("Stitchy v{} by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS")),
        "Collects a number of image files in the current directory and stitches them into a single file.".to_owned(),
        "".to_owned(),
        "Basic usage:".to_owned(),
        "".to_owned()
    ]);
    printer.print_with_wrap_indent(
        2,
        &[
            "  stitchy n".to_owned(),
            "".to_owned(),
            "  where n is the number of images to use. There must be at least that many in the \
           current directory. By default, The most recent images available will be used."
                .to_owned(),
        ],
    );
    printer.print_with_wrap_indent(0, &["".to_owned(), "Supported flags:".to_owned()]);
    printer.print_with_wrap_indent(20, &[
        "  --help            Print this help".to_owned(),
        "  --version         Print the installed version number".to_owned(),
        "  --setdefaults     Store the given options as defaults for the current user".to_owned(),
        "  --cleardefaults   Delete the default options for the current user".to_owned(),
        "  --printdefaults   Print the default options for the current user".to_owned(),
        "  --order           Select how to sort available files (default: 'latest')".to_owned(),
        "                    Allowed values: 'latest', 'alphabetic'".to_owned(),
        "  --take-from       Select which end of the sorted file list to take from (default: 'start')".to_owned(),
        "                    Allowed values: 'start', 'end'".to_owned(),
        "  --input-dir, -i   Specify a directory to source files from".to_owned(),
        "  --horizontal, -h  Force stitching across a single row only".to_owned(),
        "  --vertical, -v    Force stitching down a single column only".to_owned(),
        "  --maxw=n          Limit output width to n pixels at most".to_owned(),
        "  --maxh=n          Limit output height to n pixels at most".to_owned(),
        "  --maxd=n          Limit output width and height to n pixels at most".to_owned(),
        "  --reverse, -r     Reverse order of files, after sorting and taking from start or end".to_owned(),
        "  --quality=n       Set the output quality from 1 to 100, defaults to 100".to_owned(),
        "  --jpeg            Output as JPEG".to_owned(),
        "  --png             Output as PNG".to_owned(),
        "  --gif             Output as GIF".to_owned(),
        "  --bmp             Output as BMP".to_owned(),
        "                    Note: default format matches sources, or JPEG if source formats vary".to_owned(),
    ]);
    printer.print_with_wrap_indent(
        0,
        &[
            "".to_owned(),
            "User defaults:".to_owned(),
            "".to_owned(),
            format!(
                "Default options can be set for the current user by using the --setdefaults flag, \
        queried using the --printdefaults flag, and deleted with --cleardefaults. \
        These are stored in {} in the home directory. The next time you use Stitchy, \
        defaults will be automatically applied, though can be overridden with the same flag \
        or another flag which would perform a similar action (such as a different output format. \
        When setting defaults again, the existing ones are effectively cleared beforehand.",
                PROFILE_FILE_NAME
            ),
            "".to_owned(),
        ],
    );
}

pub fn version() {
    println!("Stitchy version {}", env!("CARGO_PKG_VERSION"));
    println!("Authored by {}", env!("CARGO_PKG_AUTHORS"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
}

pub fn defaults() {
    let load_attempt = Profile::main().into_string();
    if load_attempt.is_none() {
        println!("Did not find a {} for the current user.", PROFILE_FILE_NAME);
        return;
    }

    let json = load_attempt.unwrap();
    let serialise_result = Opt::deserialise(&json);
    if serialise_result.is_none() {
        println!(
            "Failed to parse {} for the current user.",
            PROFILE_FILE_NAME
        );
        return;
    }

    // Print JSON stored in the file
    // Deserialisation was validated above since running Stitchy will do this also
    println!("{}", json);
}
