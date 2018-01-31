use std::path::Path;

#[derive(Debug)]
pub struct BinaryObject {
    file_path: Path
}

impl std::convert::From for BinaryObject {
    
}

pub trait BinaryObjectReader {
    fn read_data(); // TODO
}
