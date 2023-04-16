
use super::util::make_size_string;

#[test]
fn check_files_length_strings() {
    let sizes: [(u64, &'static str); 6] = [
        (137, "137 bytes"),
        (1370, "1.3 KiB"),
        (13700, "13 KiB"),
        (137000, "133 KiB"),
        (1370000, "1.3 MiB"),
        (13700000, "13 MiB")
    ];
    for (size_bytes, expected_string) in sizes.into_iter() {
        let got_string = make_size_string(size_bytes);
        assert_eq!(expected_string, got_string.as_str())
    }
}
