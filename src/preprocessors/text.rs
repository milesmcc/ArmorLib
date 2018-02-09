#[macro_use] use maplit;

use std::collections::HashMap;

use preprocessor::Preprocessor;
use binary_object::BinaryObject;

struct TextPreprocessor;

impl Preprocessor for TextPreprocessor {
    fn process(binary_object: &BinaryObject) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();

        let encoding = String::from(Encoding::determine_encoding(binary_object));
        map.insert(String::from("encoding"), encoding);

        map
    }

    fn name() -> &'static str {
        "text"
    }
}

#[derive(Debug, PartialEq)]
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
        match self {
            &Encoding::Ascii => true,
            &Encoding::Utf8 => true,
            &Encoding::Utf16 => true,
            &Encoding::Utf32 => true,
            _ => false
        }
    }

    pub fn determine_encoding(binary_object: &BinaryObject) -> Encoding {
        let data: &[u8] = &binary_object.data.as_slice();

        let length = &binary_object.data.len();

        if length <= &0_usize {
            return Encoding::NoData;
        }

        if length > &4 && data[0..4] == [0xFF_u8, 0xFE_u8, 0x00_u8, 0x00_u8] {
            return Encoding::Utf32;
        } else if length > &2 && data[0..2] == [0xFF_u8, 0xFE_u8] {
            return Encoding::Utf16;
        } else if length > &3 && data[0..3] == [0xEF_u8, 0xBB_u8, 0xBF_u8] {
            return Encoding::Utf8;
        }

        if binary_object.data.iter().all(|x| *x < 128_u8) {
            return Encoding::Ascii;
        }

        Encoding::Binary
    }
}

impl From<Encoding> for String {
    fn from(encoding: Encoding) -> String {
        String::from(
            match encoding {
                Encoding::Ascii => "ascii",
                Encoding::Binary => "binary",
                Encoding::NoData => "nodata",
                Encoding::Utf16 => "utf16",
                Encoding::Utf32 => "utf32",
                Encoding::Utf8 => "utf8"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use ::preprocessors::text::*;

    use ::binary_object::BinaryObject;
    use ::util::hex_to_vec;

    #[test]
    fn test_determine_encoding() {
        // test utf32
        {
            let binary_object_utf32: BinaryObject = BinaryObject::from(hex_to_vec("FF FE 00 00 00 00 00"));
            assert_eq!(Encoding::determine_encoding(&binary_object_utf32), Encoding::Utf32);
            assert_eq!(String::from(Encoding::determine_encoding(&binary_object_utf32)), "utf32");
        }

        // test no data
        {
            let binary_object_no_data: BinaryObject = BinaryObject::from(vec![]);
            assert_eq!(Encoding::determine_encoding(&binary_object_no_data), Encoding::NoData);
            assert_eq!(String::from(Encoding::determine_encoding(&binary_object_no_data)), "nodata");
        }

        // test ascii
        {
            let binary_object_ascii: BinaryObject = BinaryObject::from(hex_to_vec("33 33 33 33 33 33 34 32 12 34"));
            assert_eq!(Encoding::determine_encoding(&binary_object_ascii), Encoding::Ascii);
            assert_eq!(String::from(Encoding::determine_encoding(&binary_object_ascii)), "ascii");
        }

        // test utf8
        {
            let binary_object_utf8: BinaryObject = BinaryObject::from(hex_to_vec("EF BB BF 00"));
            assert_eq!(Encoding::determine_encoding(&binary_object_utf8), Encoding::Utf8);
            assert_eq!(String::from(Encoding::determine_encoding(&binary_object_utf8)), "utf8");
        }

        // test utf16
        {
            let binary_object_utf16: BinaryObject = BinaryObject::from(hex_to_vec("FF FE 00 00"));
            assert_eq!(Encoding::determine_encoding(&binary_object_utf16), Encoding::Utf16);
            assert_eq!(String::from(Encoding::determine_encoding(&binary_object_utf16)), "utf16");
        }
    }

}
