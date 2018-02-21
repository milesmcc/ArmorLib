//! This module defines the BinaryObject, which represents a piece of unstructured binary data
//! of any type.

use std::fs::File;
use std::io::Read;

/// A struct that represents an unstructured piece of binary data in the form of a `Vec<u8>`.
/// It also includes the option of having a `String` to designate the path.
pub struct BinaryObject {
    /// The path on the host operating system where this `BinaryObject` resides. Because this
    /// field is not always applicible (for example, binary objects which exist only in a
    /// database), the possibility of absence is represented in the `Option`.
    pub file_path: Option<String>,

    /// A `Vec<u8>` of the binary object's data.
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
