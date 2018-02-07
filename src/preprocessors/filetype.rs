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
    NoData,
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
        if binary_object.data.iter().all(|x| *x < 128_u8) {
            return Encoding::Ascii;
        }

        if &binary_object.data.len() == &0_usize {
            return Encoding::NoData;
        }

        let data: &[u8] = &binary_object.data.as_slice();

        if data[0..4] == [0xF_b, 0xF_b, 0xF_b, 0xE_b] {
            return Encoding::Utf16;
        } else if data[0..6] == [0xE_b, 0xF_F, 0xB_b, 0xB_b, 0xB_b, 0xF_b] {
            return Encoding::Utf8;
        } else if data[0..8] == [0xF_b, 0xF_b, 0xF_b, 0xE_b, 0x0_b, 0x0_b, 0x0_b, 0x0_b] {
            return Encoding::Utf32;
        }

        Encoding::Binary
    }
}

fn determine_file_types(binary_object: &BinaryObject) -> Vec<String> {
    // first, we must determine whether the file is a text-based file or a binary
    let magic_bytes: &[u8] = &binary_object.data.as_slice()[0..8];

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_determine_encoding() {
        // test utf32
        {
            let binary_object_utf32 = BinaryObject::from(vec![0xF_u8, 0xF_u8, 0xF_u8, 0xE_u8, 0x0_u8, 0x0_u8, 0x0_u8,
                                                              0x0_u8, 0x0_u8, 0x0_u8, 0x0_u8, 0x0_u8, 0x0_u8, 0x0_u8,
                                                              0x0_u8, 0x0_u8, 0x0_u8, 0x0_u8, 0x0_u8, 0x0_u8, 0x0_u8]);
            assert_eq!(Encoding::determine_encoding(binary_object_utf32), Encoding::Utf32);
        }

        // test no data
        {
            let binary_object_no_data = BinaryObject::from(vec![]);
            assert_eq!(Encoding::determine_encoding(binary_object_no_data), Encoding::NoData);
        }

        // test ascii
        {
            let binary_object_ascii = BinaryObject::from(vec![98_u8, 86_u8, 98_u8, 86_u8, 98_u8, 86_u8, 98_u8, 86_u8]);
            assert_eq!(Encoding::determine_encoding(binary_object_ascii), Encoding::Ascii);
        }

        // test utf8
        {
            let binary_object_utf8 = BinaryObject::from(vec![0xE_b, 0xF_F, 0xB_b, 0xB_b, 0xB_b, 0xF_b, 0x0_b]);
            assert_eq!(Encoding::determine_encoding(binary_object_utf8), Encoding::Utf8);
        }

        // test utf16
        {
            let binary_object_utf16 = BinaryObject::from(vec![0xF_b, 0xF_b, 0xF_b, 0xE_b, 0x0_b, 0x0_b]);
            assert_eq!(Encoding::determine_encoding(binary_object_utf16), Encoding::Utf16);
        }
    }
}
