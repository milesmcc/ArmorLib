use std::path::Path;

#[derive(Debug)]
pub struct BinaryObject {
    file_path: Path
}

pub trait BinaryObjectReader {
    fn read_data(); // TODO
}
