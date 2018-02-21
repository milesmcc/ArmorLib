use std::fs::File;
use std::io::Read;

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

impl From<File> for BinaryObject {
    fn from(mut file: File) -> BinaryObject {
        let mut data_vec: Vec<u8> = Vec::new();
        file.read_to_end(&mut data_vec);
        BinaryObject::from(data_vec)
    }
}
