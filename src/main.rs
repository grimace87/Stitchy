pub mod image_set;

use structopt::StructOpt;
use std::time::SystemTime;
use std::path::Path;
use image_set::image_set::ImageSet;

fn main() {

    // Get command line args
    let opt = Opt::from_args();
    if opt.help {
        print_help();
        return;
    }

    // Verify a sensible number was given
    if opt.number_of_files < 2 {
        print_file_count_error();
        return;
    }

    // Iterate over files in the current directory, get all JPG and PNG images
    let accepted_extensions: [&str;5] = ["png", "jpg", "jpeg", "bmp", "gif"];
    let current_path = std::env::current_dir().unwrap();
    let mut image_files: Vec<FileData> = vec!();
    if current_path.is_dir() {
        for entry in std::fs::read_dir(current_path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                if let Some(file_extension) = path.extension() {
                    if let Some(ext_as_str) = file_extension.to_str() {
                        if accepted_extensions.contains(&ext_as_str) {
                            let useful_data = FileData{
                                full_path: path.to_str().unwrap().to_string(),
                                modify_time: path.metadata().unwrap().modified().unwrap()
                            };
                            image_files.push(useful_data);
                        }
                    }
                }
            }
        }
    }

    // Verify at least n images were found, where n is the number requested
    if image_files.len() < opt.number_of_files {
        println!("Requested {} files, found {}", opt.number_of_files, image_files.len());
        return;
    }

    // Sort files by modify date and take the most recent n files
    image_files.sort_unstable_by(|a, b| a.modify_time.cmp(&b.modify_time).reverse());
    image_files.truncate(opt.number_of_files);

    // Decode all images and keep in memory for now
    let mut image_set = ImageSet::empty_set();
    for file in image_files {
        let path = Path::new(&file.full_path);
        image_set.add_from_file_path(path);
    }

    // Prepare data set before generating output
    let new_file = Path::new("./stitch.jpg");
    image_set.generate_output_file(new_file);
    println!("Created file: {}", new_file.to_str().unwrap());
}

fn print_file_count_error() {
    println!("You must supply a number of images that is at least 2");
}

fn print_help() {
    println!("Stitchy v{} by {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    println!();
    println!("Collects all image files starting with \"Screenshot\" in");
    println!("the current directory and stitches them into a");
    println!("single file. No arguments are currently supported.");
}

#[derive(Debug, StructOpt)]
#[structopt(name = "")]
struct Opt {

    #[structopt(short, long)]
    help: bool,

    number_of_files: usize
}

struct FileData {
    full_path: String,
    modify_time: SystemTime
}
