
use crate::enums::{ImageFormat, OrderBy};
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
    let retrieved_files = retrieve_files_result.unwrap();
    let options = Opt {
        number_of_files: Some(retrieved_files.file_count()), jpeg: true, ..Opt::default()
    };
    let image_files = retrieved_files
        .sort_and_truncate_by(&options).unwrap()
        .into_image_contents().unwrap();
    let process_result = ImageSet::new(image_files, &options)
        .stitch();
    assert!(
        process_result.is_ok(),
        "{}", process_result.err().unwrap_or(String::new()));
}

#[test]
fn mixin_quality_ignored_for_png_override() {

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
    let retrieved_files = retrieve_files_result.unwrap();
    let loaded_defaults = Opt { number_of_files: Some(1), jpeg: true, quality: 50, ..Opt::default() };
    let options = Opt { number_of_files: Some(retrieved_files.file_count()), png: true, ..Opt::default() }
        .mix_in(&loaded_defaults);
    let image_files = retrieved_files
        .sort_and_truncate_by(&options).unwrap()
        .into_image_contents().unwrap();
    let process_result = ImageSet::new(image_files, &options)
        .stitch();
    assert!(
        process_result.is_ok(),
        "{}", process_result.err().unwrap_or(String::new()));
}

#[test]
pub fn test_unusual_inputs() {

    // Clear existing file
    let clear_result = clear_output();
    assert!(
        clear_result.is_ok(),
        "{}", clear_result.err().unwrap_or(String::new()));

    // Get files from test directory
    let retrieve_files_result =
        ImageFiles::from_directory(vec!("images", "testing", "test_unusual_inputs"));
    assert!(
        retrieve_files_result.is_ok(),
        "{}", retrieve_files_result.err().unwrap_or(String::new()));

    // Unpack input images, confirm correct number
    let retrieved_files = retrieve_files_result.unwrap();
    assert_eq!(retrieved_files.file_count(), 2);

    // Process files, generate output
    let options = Opt { number_of_files: Some(retrieved_files.file_count()), jpeg: true, ..Opt::default() };
    let image_files = retrieved_files
        .sort_and_truncate_by(&options).unwrap()
        .into_image_contents().unwrap();
    let process_result = ImageSet::new(image_files, &options)
        .stitch();
    assert!(
        process_result.is_ok(),
        "{}", process_result.err().unwrap_or(String::new()));
}

#[test]
pub fn test_output_dimensions() {

    // Clear existing file
    let clear_result = clear_output();
    assert!(
        clear_result.is_ok(),
        "{}", clear_result.err().unwrap_or(String::new()));

    // Stitch first 3 files
    let options = Opt {
        number_of_files: Some(3), horizontal: true, order: Some(OrderBy::Alphabetic),
        ..Opt::default() };
    let image_files = ImageFiles::
        from_directory(vec!("images", "testing", "test_output_dimensions")).unwrap()
        .sort_and_truncate_by(&options).unwrap()
        .into_image_contents().unwrap();
    let process_result = ImageSet::new(image_files, &options).stitch().unwrap();

    // Assert dimensions
    assert_eq!(process_result.width(), 3240);
    assert_eq!(process_result.height(), 2280);
}

#[test]
pub fn test_file_counts() {

    // Attempt increasing number of files, from 2 to 10
    for i in 2..11 {

        // Clear existing file
        let clear_result = clear_output();
        assert!(
            clear_result.is_ok(),
            "{}", clear_result.err().unwrap_or(String::new()));

        // Get files from test directory
        let retrieve_files_result = ImageFiles::from_directory(
            vec!("images", "testing", "test_file_counts"));
        assert!(
            retrieve_files_result.is_ok(),
            "{}", retrieve_files_result.err().unwrap_or(String::new()));

        // Process files, generate output
        let options = Opt { number_of_files: Some(i), jpeg: true, ..Opt::default() };
        let image_files = retrieve_files_result.unwrap()
            .sort_and_truncate_by(&options).unwrap()
            .into_image_contents().unwrap();
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

        // Build options set matching the image format under test
        let retrieved_files = retrieve_files_result.unwrap();
        let all_files_count = retrieved_files.file_count();
        let format = ImageFormat::infer_format(extension);
        let options = match format {
            ImageFormat::Jpeg => Opt {
                number_of_files: Some(all_files_count), jpeg: true, ..Opt::default()
            },
            ImageFormat::Png => Opt {
                number_of_files: Some(all_files_count), png: true, ..Opt::default()
            },
            ImageFormat::Bmp => Opt {
                number_of_files: Some(all_files_count), bmp: true, ..Opt::default()
            },
            ImageFormat::Gif => Opt {
                number_of_files: Some(all_files_count), gif: true, ..Opt::default()
            },
            ImageFormat::Unspecified => Opt {
                number_of_files: Some(all_files_count), ..Opt::default()
            }
        };

        // Process input files
        let image_files = retrieved_files
            .sort_and_truncate_by(&options).unwrap()
            .into_image_contents().unwrap();

        // Perform stitch on inputs
        let process_result = ImageSet::new(image_files, &options)
            .stitch();
        assert!(
            process_result.is_ok(),
            "{}", process_result.err().unwrap_or(String::new()));
    }
}
