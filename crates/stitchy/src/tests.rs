
use crate::Opt;
use stitchy_core::{ImageFiles, FilePathWithMetadata, image::ImageFormat, OrderBy, TakeFrom, Stitch, extension_formats};

fn clear_output() -> Result<(), String> {
    let current_path = std::env::current_dir().unwrap();
    assert!(current_path.is_dir());
    let mut test_file = current_path.clone();
    test_file.push("test.jpg");
    return if test_file.is_file() {
        std::fs::remove_file(test_file.as_path()).map_err(
            |e| format!("Previous test file exists but couldn't be removed: {}", e)
        )
    } else {
        Ok(())
    }
}

#[test]
pub fn test_output_formats() {

    // Per allowed extension, infer the type enum and generate an output
    for &extension in ImageFiles::<FilePathWithMetadata>::allowed_extensions().iter() {

        // Clear existing file
        let clear_result = clear_output();
        assert!(
            clear_result.is_ok(),
            "{}", clear_result.err().unwrap_or(String::new()));

        // Get files from test directory
        let retrieve_files_result = ImageFiles::builder()
            .add_current_directory(vec!("..", "..", "images", "testing", "test_output_formats")).unwrap()
            .build();
        assert!(
            retrieve_files_result.is_ok(),
            "{}", retrieve_files_result.err().unwrap_or(String::new()));

        // Build options set matching the image format under test
        let retrieved_files = retrieve_files_result.unwrap();
        let all_files_count = retrieved_files.file_count();
        let mut format: Option<ImageFormat> = None;
        for &(ext, fmt) in extension_formats().iter() {
            if ext == extension {
                format = Some(fmt);
            }
        }
        let options = match format {
            Some(ImageFormat::Jpeg) => Opt {
                number_of_files: Some(all_files_count), jpeg: true, ..Opt::default()
            },
            Some(ImageFormat::Png) => Opt {
                number_of_files: Some(all_files_count), png: true, ..Opt::default()
            },
            Some(ImageFormat::Bmp) => Opt {
                number_of_files: Some(all_files_count), bmp: true, ..Opt::default()
            },
            Some(ImageFormat::WebP) => Opt {
                number_of_files: Some(all_files_count), webp: true, ..Opt::default()
            },
            Some(ImageFormat::Gif) => Opt {
                number_of_files: Some(all_files_count), gif: true, ..Opt::default()
            },
            Some(inner_format) => { panic!("Unknown format: {:?}", inner_format) },
            None => Opt {
                number_of_files: Some(all_files_count), ..Opt::default()
            }
        };

        // Process input files
        let sources = retrieved_files
            .sort_and_truncate_by(
                options.number_of_files.unwrap(),
                OrderBy::Latest,
                TakeFrom::Start,
                false
            ).unwrap();
        let output_path = crate::file_util::next_available_output(&sources, &options).unwrap();
        let image_files = sources.into_image_contents(false).unwrap();

        // Perform stitch on inputs
        let stitch = Stitch::builder()
            .images(image_files)
            .stitch().unwrap();
        let process_result = crate::file_util::write_image_to_file(stitch, &output_path, format, 100);
        assert!(
            process_result.is_ok(),
            "{}", process_result.err().unwrap_or(String::new()));
    }
}
