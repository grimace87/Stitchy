
const BYTES_KIB: u64 = 1024;
const BYTES_MIB: u64 = 1024 * 1024;

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
