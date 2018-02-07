use std::collections::HashMap;
use std::io::Read;

use preprocessor::Preprocessor;
use binary_object::BinaryObject;

struct FiletypePrepreprocessor;

pub enum Encoding {
    Ascii,
    Utf8,
    Utf16,
    Utf32,
    Binary
}

impl Encoding {
    pub fn is_text(&self) -> bool {
        match &self {
            &Ascii => true,
            &Utf8 => true,
            &Utf16 => true,
            &Utf32 => true,
            _ => false
        }
    }

    pub fn determine_encoding(binary_object: &BinaryObject) -> Encoding {
        let data: &Read = binary_object.clone().data;
        let magic_bytes: &mut [u8] = &mut [8];
        data.

        // TODO: how to read data if Read trait requires mutability?

        Encoding::Binary
    }
}

fn determine_file_types(binary_object: &BinaryObject) -> Vec<String> {
    // first, we must determine whether the file is a text-based file or a binary
    let data: &Read = &binary_object.data;
    let magic_bytes: &mut [u8] = &mut [8];
    data.read_exact(magic_bytes);

    let encoding = Encoding::determine_encoding(binary_object);

    return Vec::new();
}

impl Preprocessor for FiletypePrepreprocessor {

    /// Returns a map where the keys are file types (without the leading `.`).
    /// Values are not currently used.
    fn process(binary_object: &BinaryObject) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        for filetype in determine_file_types(binary_object).iter() {
            map.insert(filetype.clone(), String::from(""));
        };
        return map
    }

    fn name() -> &'static str {
        "filetype"
    }
}
