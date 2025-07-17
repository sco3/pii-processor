/// preview for logging. returns beginning of possibly long string or bytes
use bytes::Bytes;
/// preview first 80 bytes
#[must_use]
pub fn preview(file_content: &[u8]) -> Bytes {
    let data = &file_content[..file_content.len().min(80)];

    Bytes::copy_from_slice(data)
}
/// preview first 80 bytes
pub fn preview_bytes(file_content: &Bytes) -> Bytes {
    file_content.slice(..file_content.len().min(80))
}
/// preview first 80 chars
#[must_use]
pub fn preview_str(content: &str) -> String {
    content.chars().take(80).collect()
}
