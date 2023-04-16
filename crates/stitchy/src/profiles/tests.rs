
use super::Profile;

#[test]
fn text_written_reads_back() {
    let test_content = "{\"greeting\":\"Ahoy!\"}".to_owned();
    Profile::test_file().write_string(test_content.clone());
    let retrieved = Profile::test_file().into_string()
        .expect("Could not read written file");
    assert_eq!(test_content, retrieved);
}
