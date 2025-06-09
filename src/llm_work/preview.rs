use bytes::Bytes;

pub fn preview(file_content: &[u8]) -> Bytes {
    let data = &file_content[..file_content.len().min(80)];

    Bytes::copy_from_slice(data)
}
pub fn preview_bytes(file_content: &Bytes) -> Bytes {
    file_content.slice(..file_content.len().min(80))
}
