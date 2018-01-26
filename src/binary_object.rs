use std::path::Path;

pub struct BinaryObject {
    file_path: Path
}

pub trait BinaryObjectReader {
    fn read_data(); // TODO
}
