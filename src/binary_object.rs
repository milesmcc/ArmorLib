pub struct BinaryObject {
    pub file_name: Option<String>,
    pub data: Vec<u8>
}

fn from(data: Vec<u8>) -> BinaryObject {
    BinaryObject {
        file_name: None,
        data: data
    }
}
