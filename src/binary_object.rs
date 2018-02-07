use std::io::Read;

#[derive(Copy)]
pub struct BinaryObject {
    pub file_name: Option<String>,
    pub data: Read
}
