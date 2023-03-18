use crate::enums::ImageFormat;
use crate::files::ImageFiles;
use crate::image_set::ImageSet;
use crate::options::Opt;

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
pub fn test_types() {

    // Clear existing file
    let clear_result = clear_output();
    assert!(
        clear_result.is_ok(),
        "{}", clear_result.err().unwrap_or(String::new()));

    // Get files from test directory
    let retrieve_files_result =
        ImageFiles::from_directory(vec!("images", "testing", "test_types"));
    assert!(
        retrieve_files_result.is_ok(),
        "{}", retrieve_files_result.err().unwrap_or(String::new()));

    // Process files, generate output
    let image_files = retrieve_files_result.unwrap()
        .into_image_contents().unwrap();
    let options = Opt { number_of_files: Some(image_files.len()), jpeg: true, ..Opt::default() };
    let process_result = ImageSet::new(image_files, &options)
        .stitch();
    assert!(
        process_result.is_ok(),
        "{}", process_result.err().unwrap_or(String::new()));
}

#[test]
pub fn test_sizes() {

    // Attempt increasing number of files, from 2 to 10
    for i in 2..11 {

        // Clear existing file
        let clear_result = clear_output();
        assert!(
            clear_result.is_ok(),
            "{}", clear_result.err().unwrap_or(String::new()));

        // Get files from test directory
        let retrieve_files_result = ImageFiles::from_directory(
            vec!("images", "testing", "test_sizes"));
        assert!(
            retrieve_files_result.is_ok(),
            "{}", retrieve_files_result.err().unwrap_or(String::new()));

        // Process files, generate output
        let image_files = retrieve_files_result.unwrap()
            .into_image_contents().unwrap();
        let options = Opt { number_of_files: Some(i), jpeg: true, ..Opt::default() };
        let process_result = ImageSet::new(image_files, &options)
            .stitch();
        assert!(
            process_result.is_ok(),
            "{}", process_result.err().unwrap_or(String::new()));
    }
}

#[test]
pub fn test_output_formats() {

    // Per allowed extension, infer the type enum and generate an output
    for &extension in ImageFormat::allowed_extensions().iter() {

        // Clear existing file
        let clear_result = clear_output();
        assert!(
            clear_result.is_ok(),
            "{}", clear_result.err().unwrap_or(String::new()));

        // Get files from test directory
        let retrieve_files_result = ImageFiles::from_directory(
            vec!("images", "testing", "test_output_formats"));
        assert!(
            retrieve_files_result.is_ok(),
            "{}", retrieve_files_result.err().unwrap_or(String::new()));

        // Process input files
        let image_files = retrieve_files_result.unwrap()
            .into_image_contents().unwrap();

        // Build options set matching the image format under test
        let format = ImageFormat::infer_format(extension);
        let options = match format {
            ImageFormat::Jpeg => Opt {
                number_of_files: Some(image_files.len()), jpeg: true, ..Opt::default()
            },
            ImageFormat::Png => Opt {
                number_of_files: Some(image_files.len()), png: true, ..Opt::default()
            },
            ImageFormat::Bmp => Opt {
                number_of_files: Some(image_files.len()), bmp: true, ..Opt::default()
            },
            ImageFormat::Gif => Opt {
                number_of_files: Some(image_files.len()), gif: true, ..Opt::default()
            },
            ImageFormat::Unspecified => Opt {
                number_of_files: Some(image_files.len()), ..Opt::default()
            }
        };

        // Perform stitch on inputs
        let process_result = ImageSet::new(image_files, &options)
            .stitch();
        assert!(
            process_result.is_ok(),
            "{}", process_result.err().unwrap_or(String::new()));
    }
}
