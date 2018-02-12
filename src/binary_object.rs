pub struct BinaryObject {
    pub file_path: Option<String>,
    pub data: Vec<u8>,
}

impl From<Vec<u8>> for BinaryObject {
    fn from(data: Vec<u8>) -> BinaryObject {
        BinaryObject {
            file_path: None,
            data: data,
        }
    }
}
