pub mod builder;
pub mod image;
pub mod util;

use std::time::SystemTime;

pub(crate) struct FileProperties {
    full_path: String,
    modify_time: SystemTime,
    size_bytes: u64
}
