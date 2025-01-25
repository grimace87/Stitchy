
use image::ImageFormat;

const BYTES_KIB: u64 = 1024;
const BYTES_MIB: u64 = 1024 * 1024;

/// Return a string representing the size of the file, in bytes, KiB, or MiB
///
/// Used in [crate::ImageFileSet::into_image_contents] if outputting information about the input
/// files, and shared with the CLI crate to print output file size in the same format.
pub fn make_size_string(length_bytes: u64) -> String {
    match length_bytes {
        l if l < BYTES_KIB => format!(
            "{} bytes", l
        ),
        l if l < 10 * BYTES_KIB => format!(
            "{}.{} KiB", l / BYTES_KIB, (10 * (l % BYTES_KIB)) / BYTES_KIB
        ),
        l if l < BYTES_MIB => format!(
            "{} KiB", l / BYTES_KIB
        ),
        l if l < 10 * BYTES_MIB => format!(
            "{}.{} MiB", l / BYTES_MIB, (10 * (l % BYTES_MIB)) / BYTES_MIB
        ),
        l => format!("{} MiB", l / BYTES_MIB)
    }
}

/// Mappings of known file extensions to their image format
pub fn extension_formats() -> [(&'static str, ImageFormat); 6] {
    [
        (".jpg", ImageFormat::Jpeg),
        (".jpeg", ImageFormat::Jpeg),
        (".png", ImageFormat::Png),
        (".gif", ImageFormat::Gif),
        (".bmp", ImageFormat::Bmp),
        (".webp", ImageFormat::WebP)
    ]
}
