use crate::AlignmentMode;
use crate::enums::{OrderBy, TakeFrom};
use crate::files::ImageFiles;
use crate::stitch::Stitch;

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
        ImageFiles::from_current_directory(vec!("..", "..", "images", "testing", "test_types"));
    assert!(
        retrieve_files_result.is_ok(),
        "{}", retrieve_files_result.err().unwrap_or(String::new()));

    // Process files, generate output
    let image_files = retrieve_files_result.unwrap()
        .into_image_contents(false).unwrap();
    let process_result = Stitch::begin()
        .images(image_files)
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
        ImageFiles::from_current_directory(vec!("..", "..", "images", "testing", "test_unusual_inputs"));
    assert!(
        retrieve_files_result.is_ok(),
        "{}", retrieve_files_result.err().unwrap_or(String::new()));

    // Unpack input images, confirm correct number
    let retrieved_files = retrieve_files_result.unwrap();
    assert_eq!(retrieved_files.file_count(), 2);

    // Process files, generate output
    let image_files = retrieved_files
        .into_image_contents(false).unwrap();
    let process_result = Stitch::begin()
        .images(image_files)
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

    // Stitch first 3 files horizontally
    // Trivial case of 3 identically-sized images of 1080 x 2280 each
    // Expect output width 1080 x 3 and height 1080
    let image_files = ImageFiles::
        from_current_directory(vec!("..", "..", "images", "testing", "test_output_dimensions")).unwrap()
        .sort_and_truncate_by(3, OrderBy::Alphabetic, TakeFrom::Start, false).unwrap()
        .into_image_contents(false).unwrap();
    let process_result = Stitch::begin()
        .images(image_files)
        .alignment(AlignmentMode::Horizontal)
        .stitch().unwrap();

    // Assert dimensions
    assert_eq!(process_result.width(), 3240);
    assert_eq!(process_result.height(), 2280);

    // Stitch last 3 files horizontally in reverse order
    // One image of 1080 x 1080, then two of 1080 x 2280 which must scale down to line up
    // Expect images scaled down to be 511 wide (1080 x 1080 / 2280 = 511.5789 which we round
    // down) hence overall output width of 2 x 511 + 1080 = 2102
    let image_files = ImageFiles::
        from_current_directory(vec!("..", "..", "images", "testing", "test_output_dimensions")).unwrap()
        .sort_and_truncate_by(3, OrderBy::Alphabetic, TakeFrom::End, false).unwrap()
        .into_image_contents(false).unwrap();
    let process_result = Stitch::begin()
        .images(image_files)
        .alignment(AlignmentMode::Horizontal)
        .stitch().unwrap();

    // Assert dimensions
    assert_eq!(process_result.width(), 2102);
    assert_eq!(process_result.height(), 1080);

    // Stitch all 4 files in a grid
    // Main axis will default to horizontal, so images go across first row then across a second,
    // hence scaled-down images give overall width of 511 + 1080 (from bottom row) and height of
    // 2 x 1080
    // NOTE these images could be stitched together in a smarter way!
    let image_files = ImageFiles::
        from_current_directory(vec!("..", "..", "images", "testing", "test_output_dimensions")).unwrap()
        .sort_and_truncate_by(4, OrderBy::Alphabetic, TakeFrom::Start, false).unwrap()
        .into_image_contents(false).unwrap();
    let process_result = Stitch::begin()
        .images(image_files)
        .stitch().unwrap();

    // Assert dimensions
    assert_eq!(process_result.width(), 1591);
    assert_eq!(process_result.height(), 2160);
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
        let retrieve_files_result = ImageFiles::from_current_directory(
            vec!("..", "..", "images", "testing", "test_file_counts"));
        assert!(
            retrieve_files_result.is_ok(),
            "{}", retrieve_files_result.err().unwrap_or(String::new()));

        // Process files, generate output
        let image_files = retrieve_files_result.unwrap()
            .sort_and_truncate_by(i, OrderBy::Latest, TakeFrom::Start, false).unwrap()
            .into_image_contents(false).unwrap();
        let process_result = Stitch::begin()
            .images(image_files)
            .stitch();
        assert!(
            process_result.is_ok(),
            "{}", process_result.err().unwrap_or(String::new()));
    }
}
