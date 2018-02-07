use std::io::Read;

pub struct BinaryObject {
    file_name: Option<String>,
    bytes: Read
}
