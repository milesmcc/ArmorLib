use std::path::Path;
use std::convert::From;
use std::io::Bytes;
use std::io::Read;

#[derive(Debug)]
pub struct BinaryObject {
    bytes: Read,
}

pub trait BinaryObjectReader {
    fn read_data(); // TODO
}
