pub mod builder;
pub mod image;
pub mod util;

use std::time::SystemTime;

/// Wrapper for file metadata.
/// Loading these properties for a file does not require actually reading, let alone parsing, the
/// file's contents.
pub(crate) struct FileProperties {
    full_path: String,
    modify_time: SystemTime,
    size_bytes: u64
}
