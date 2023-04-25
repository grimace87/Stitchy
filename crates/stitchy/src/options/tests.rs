
use crate::{Opt, TakeFrom, OrderBy};
use stitchy_core::ImageFiles;

const TEST_JSON: &str = "{ \
        \"horizontal\":true, \
        \"vertical\":false, \
        \"maxd\":0, \
        \"maxw\":120, \
        \"maxh\":0, \
        \"reverse\":false, \
        \"take_from\":null, \
        \"jpeg\":true, \
        \"png\":false, \
        \"gif\":false, \
        \"bmp\":false, \
        \"quality\":80, \
        \"order\":null, \
        \"number_of_files\":null \
        }";

fn trim_all(s: &str) -> String {
    s.split_whitespace().collect()
}

fn make_test_default() -> Opt {
    Opt {
        number_of_files: Some(1),
        ..Opt::default()
    }
}

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
fn deserializes_okay() {
    let expected = Opt {
        horizontal: true,
        maxw: 120,
        jpeg: true,
        quality: 80,
        number_of_files: None,
        ..Opt::default()
    };
    let result = Opt::deserialise(TEST_JSON);
    assert!(result.is_some());
    let opt = result.unwrap();
    assert_eq!(format!("{:?}", expected), format!("{:?}", opt));
}

#[test]
fn serializes_okay() {
    let from_opt = Opt {
        horizontal: true,
        maxw: 120,
        jpeg: true,
        quality: 80,
        number_of_files: None,
        ..Opt::default()
    };
    let result = from_opt.serialise();
    assert!(result.is_some());
    let json = result.unwrap();
    let trimmed_json = trim_all(json.as_str());
    let expected = trim_all(TEST_JSON);
    assert_eq!(expected, trimmed_json);
}

#[test]
fn test_default_is_valid() {
    let error = make_test_default().check_for_basic_errors(&None);
    assert!(error.is_none());
}

#[test]
fn default_quailty_is_100() {
    let opt = make_test_default();
    assert_eq!(opt.quality, super::DEFAULT_QUALITY);
}

#[test]
fn choosing_both_directions_gives_error() {
    let error = Opt { horizontal: true, vertical: true, ..make_test_default() }
        .check_for_basic_errors(&None);
    assert!(error.is_some());
}

#[test]
fn choosing_neither_direction_gives_no_error() {
    let opt = make_test_default();
    assert_eq!(opt.horizontal, false);
    assert_eq!(opt.vertical, false);
    let error = opt.check_for_basic_errors(&None);
    assert!(error.is_none());
}

#[test]
fn setting_general_and_specific_dimension_constraints_gives_error() {
    let error_1 = Opt { maxd: 100, maxw: 100, ..make_test_default() }.check_for_basic_errors(&None);
    let error_2 = Opt { maxd: 100, maxh: 100, ..make_test_default() }.check_for_basic_errors(&None);
    let error_3 = Opt { maxd: 100, maxw: 100, maxh: 100, ..make_test_default() }
        .check_for_basic_errors(&None);
    assert!(error_1.is_some());
    assert!(error_2.is_some());
    assert!(error_3.is_some());
}

#[test]
fn setting_both_specific_dimension_constraints_gives_no_error() {
    let error = Opt { maxw: 100, maxh: 100, ..make_test_default() }.check_for_basic_errors(&None);
    assert!(error.is_none());
}

#[test]
fn setting_zero_dimension_constraints_gives_no_error() {
    let error = Opt { maxd: 0, maxw: 0, maxh: 0, ..make_test_default() }
        .check_for_basic_errors(&None);
    assert!(error.is_none());
}

#[test]
fn choosing_multiple_formats_gives_error() {
    let error_1 = Opt { jpeg: true, png: true, ..make_test_default() }.check_for_basic_errors(&None);
    let error_2 = Opt { png: true, gif: true, ..make_test_default() }.check_for_basic_errors(&None);
    let error_3 = Opt { gif: true, bmp: true, ..make_test_default() }.check_for_basic_errors(&None);
    let error_4 = Opt { jpeg: true, png: true, gif: true, bmp: true, ..make_test_default() }
        .check_for_basic_errors(&None);
    assert!(error_1.is_some());
    assert!(error_2.is_some());
    assert!(error_3.is_some());
    assert!(error_4.is_some());
}

#[test]
fn choosing_no_format_gives_no_error() {
    let opt = Opt { ..make_test_default() };
    assert_eq!(opt.jpeg, false);
    assert_eq!(opt.png, false);
    assert_eq!(opt.gif, false);
    assert_eq!(opt.bmp, false);
    let error = opt.check_for_basic_errors(&None);
    assert!(error.is_none());
}

#[test]
fn choosing_quality_for_non_jpeg_gives_error() {
    let error_1 = Opt { png: true, quality: 50, ..make_test_default() }
        .check_for_basic_errors(&None);
    let error_2 = Opt { gif: true, quality: 50, ..make_test_default() }
        .check_for_basic_errors(&None);
    let error_3 = Opt { bmp: true, quality: 50, ..make_test_default() }
        .check_for_basic_errors(&None);
    assert!(error_1.is_some());
    assert!(error_2.is_some());
    assert!(error_3.is_some());
}

#[test]
fn choosing_quality_for_jpeg_gives_no_error() {
    let error = Opt { jpeg: true, quality: 50, ..make_test_default() }.check_for_basic_errors(&None);
    assert!(error.is_none());
}

#[test]
fn choosing_silly_quality_gives_error() {
    let error = Opt { jpeg: true, quality: 250, ..make_test_default() }.check_for_basic_errors(&None);
    assert!(error.is_some());
}

#[test]
fn base_options_favoured_in_classes() {
    // Classes such as dimension constraints, image format, sort order, etc.
    let base = Opt {
        horizontal: true,
        maxw: 540,
        png: true,
        order: Some(OrderBy::Latest),
        take_from: Some(TakeFrom::End),
        ..Opt::default()
    };
    let mixer = Opt {
        vertical: true,
        maxd: 540,
        gif: true,
        order: Some(OrderBy::Alphabetic),
        take_from: Some(TakeFrom::Start),
        ..Opt::default()
    };
    let merged = base.mix_in(&mixer);
    assert_eq!(merged.horizontal, true);
    assert_eq!(merged.vertical, false);
    assert_eq!(merged.maxd, 0);
    assert_eq!(merged.maxw, 540);
    assert_eq!(merged.maxh, 0);
    assert_eq!(merged.jpeg, false);
    assert_eq!(merged.png, true);
    assert_eq!(merged.gif, false);
    assert_eq!(merged.bmp, false);
    assert_eq!(merged.order, Some(OrderBy::Latest));
    assert_eq!(merged.take_from, Some(TakeFrom::End));
}

#[test]
fn mixin_preserves_mixer_booleans() {
    let mixer = Opt {
        horizontal: true,
        vertical: true,
        reverse: true,
        jpeg: true,
        png: true,
        gif: true,
        bmp: true,
        ..Opt::default()
    };
    let merged = Opt::default().mix_in(&mixer);
    assert!(merged.horizontal);
    assert!(merged.vertical);
    assert!(merged.reverse);
    assert!(merged.jpeg);
    assert!(merged.png);
    assert!(merged.gif);
    assert!(merged.bmp);
}

#[test]
fn mixin_preserves_original_booleans() {
    let base = Opt {
        horizontal: true,
        vertical: true,
        reverse: true,
        jpeg: true,
        png: true,
        gif: true,
        bmp: true,
        ..Opt::default()
    };
    let merged = base.mix_in(&Opt::default());
    assert!(merged.horizontal);
    assert!(merged.vertical);
    assert!(merged.reverse);
    assert!(merged.jpeg);
    assert!(merged.png);
    assert!(merged.gif);
    assert!(merged.bmp);
}

#[test]
fn mixin_preserves_mixer_integers() {
    let mixer = Opt {
        maxd: 100,
        maxw: 200,
        maxh: 50,
        quality: 80,
        ..Opt::default()
    };
    let merged = Opt::default().mix_in(&mixer);
    assert_eq!(merged.maxd, 100);
    assert_eq!(merged.maxw, 200);
    assert_eq!(merged.maxh, 50);
    assert_eq!(merged.quality, 80);
}

#[test]
fn mixin_preserves_original_integers() {
    let base = Opt {
        maxd: 100,
        maxw: 200,
        maxh: 50,
        quality: 80,
        ..Opt::default()
    };
    let merged = base.mix_in(&Opt::default());
    assert_eq!(merged.maxd, 100);
    assert_eq!(merged.maxw, 200);
    assert_eq!(merged.maxh, 50);
    assert_eq!(merged.quality, 80);
}

#[test]
fn mixin_favours_original_integers() {
    let base = Opt {
        maxd: 100,
        maxw: 200,
        maxh: 50,
        quality: 80,
        ..Opt::default()
    };
    let mixer = Opt {
        maxd: 50,
        maxw: 100,
        maxh: 25,
        quality: 40,
        ..Opt::default()
    };
    let merged = base.mix_in(&mixer);
    assert_eq!(merged.maxd, 100);
    assert_eq!(merged.maxw, 200);
    assert_eq!(merged.maxh, 50);
    assert_eq!(merged.quality, 80);
}

#[test]
fn mixin_preserves_non_default_quality() {
    let base = Opt {
        quality: super::DEFAULT_QUALITY,
        ..Opt::default()
    };
    let mixer = Opt {
        quality: 40,
        ..Opt::default()
    };
    let merged = base.mix_in(&mixer);
    assert_eq!(merged.quality, 40);
}

#[test]
fn mixin_preserves_mixer_enums() {
    let base = Opt::default();
    let mixer = Opt {
        take_from: Some(TakeFrom::End),
        order: Some(OrderBy::Alphabetic),
        ..Opt::default()
    };
    let merged = base.mix_in(&mixer);
    assert_eq!(merged.take_from, Some(TakeFrom::End));
    assert_eq!(merged.order, Some(OrderBy::Alphabetic));
}

#[test]
fn mixin_preserves_original_enums() {
    let base = Opt {
        take_from: Some(TakeFrom::End),
        order: Some(OrderBy::Alphabetic),
        ..Opt::default()
    };
    let mixer = Opt::default();
    let merged = base.mix_in(&mixer);
    assert_eq!(merged.take_from, Some(TakeFrom::End));
    assert_eq!(merged.order, Some(OrderBy::Alphabetic));
}

#[test]
fn mixin_preserves_some_number_of_files() {
    let base = Opt { number_of_files: Some(5), ..Opt::default() };
    let mixer = Opt { number_of_files: None, ..Opt::default() };
    let merged_1 = base.mix_in(&mixer);
    let base = Opt { number_of_files: None, ..Opt::default() };
    let mixer = Opt { number_of_files: Some(7), ..Opt::default() };
    let merged_2 = base.mix_in(&mixer);
    let base = Opt { number_of_files: None, ..Opt::default() };
    let mixer = Opt { number_of_files: None, ..Opt::default() };
    let merged_3 = base.mix_in(&mixer);
    assert!(merged_1.number_of_files.is_some());
    assert!(merged_2.number_of_files.is_some());
    assert!(merged_3.number_of_files.is_none());
}

#[test]
fn mixin_favours_original_number_of_files() {
    let base = Opt { number_of_files: Some(5), ..Opt::default() };
    let mixer = Opt { number_of_files: Some(7), ..Opt::default() };
    let merged_number = base.mix_in(&mixer).number_of_files.unwrap();
    assert_eq!(merged_number, 5);
}

#[test]
fn bad_json_does_not_deserialise() {
    let test_str = "{\"name\":\"Options\"}";
    let options = Opt::deserialise_as_current(test_str);
    assert!(options.is_none());
}

#[test]
fn v1_options_does_deserialise() {
    let test_str = "{\"horizontal\":true,\"vertical\":false,\"maxd\":0,\"maxw\":0,\"maxh\":0\
        ,\"reverse\":false,\"jpeg\":false,\"png\":true,\"gif\":false,\"bmp\":false,\"quality\":100\
        ,\"ascalpha\":true,\"descalpha\":false,\"number_of_files\":null}";
    let options = Opt::deserialise_as_current(test_str);
    assert!(options.is_some());
}

#[test]
fn mixin_quality_ignored_for_png_override() {

    // Clear existing file
    let clear_result = clear_output();
    assert!(
        clear_result.is_ok(),
        "{}", clear_result.err().unwrap_or(String::new()));

    // Get files from test directory
    let retrieve_files_result = ImageFiles::builder()
        .add_current_directory(vec!("..", "..", "images", "testing", "test_types")).unwrap()
        .build();
    assert!(
        retrieve_files_result.is_ok(),
        "{}", retrieve_files_result.err().unwrap_or(String::new()));

    // Process files, generate output
    let retrieved_files = retrieve_files_result.unwrap();
    let loaded_defaults = Opt { number_of_files: Some(1), jpeg: true, quality: 50, ..Opt::default() };
    let options = Opt { number_of_files: Some(retrieved_files.file_count()), png: true, ..Opt::default() }
        .mix_in(&loaded_defaults);
    let post_mix_error = options.check_for_basic_errors(&Some(loaded_defaults));
    assert!(
        post_mix_error.is_none(),
        "{}", post_mix_error.unwrap_or(String::new()));
}
