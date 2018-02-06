use std::path::Path;
use std::convert::From;

#[derive(Debug)]
pub struct BinaryObject {
    file_path: Path
}

pub trait BinaryObjectReader {
    fn read_data(); // TODO
}
