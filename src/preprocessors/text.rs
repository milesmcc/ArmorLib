use std::collections::HashMap;
use std::char;
use std::u16;

use preprocessor::Preprocessor;
use binary_object::BinaryObject;

use util;

pub struct TextPreprocessor;

impl Preprocessor for TextPreprocessor {
    /// Creates two fields: encoding and text
    /// TODO: improve documentation; see https://github.com/milesmcc/ArmorLib/issues/8
    fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();

        // determine encoding
        let encoding = Encoding::determine_encoding(binary_object);
        map.insert(String::from("encoding"), String::from(encoding));

        // extract text
        map.insert(String::from("text"), encoding.extract_text(binary_object));

        map
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("text", "intelligently extracts strings")
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Encoding {
    Ascii,
    Utf8,
    Utf16,
    Utf32,
    NoData,
    Binary,
}

impl Encoding {
    pub fn is_text(&self) -> bool {
        match self {
            &Encoding::Ascii => true,
            &Encoding::Utf8 => true,
            &Encoding::Utf16 => true,
            &Encoding::Utf32 => true,
            _ => false,
        }
    }

    pub fn determine_encoding(binary_object: &BinaryObject) -> Encoding {
        let data: &[u8] = &binary_object.data.as_slice();

        let length = &binary_object.data.len();

        if length <= &0_usize {
            return Encoding::NoData;
        }

        if length > &4_usize && data[0..4] == [0xFF_u8, 0xFE_u8, 0x00_u8, 0x00_u8] {
            // could also do % 4 check, but assumes files aren't corrupted
            return Encoding::Utf32;
        } else if length > &2_usize && data[0..2] == [0xFF_u8, 0xFE_u8] {
            // could also do % 2 check, see above
            return Encoding::Utf16;
        } else if length > &3_usize && data[0..3] == [0xEF_u8, 0xBB_u8, 0xBF_u8] {
            return Encoding::Utf8;
        }

        if data.iter().all(|x| *x < 128_u8) {
            return Encoding::Ascii;
        }

        // in case the magic number for utf8 is not present
        if String::from_utf8(data.to_vec()).is_ok() {
            return Encoding::Utf8;
        }

        Encoding::Binary
    }

    /// Extract the given binary object's data into a String. Some string will be generated for
    /// any and all possible binary objects. It may contain invalid characters.
    pub fn extract_text(&self, binary_object: &BinaryObject) -> String {
        let data = &binary_object.data;
        let data_slice = data.as_slice();

        match self {
            &Encoding::Ascii => String::from(String::from_utf8_lossy(data.as_slice()).to_owned()),
            &Encoding::Utf16 => {
                let mut len = data.len();
                if len % 2 != 0 {
                    len -= 1; // truncate the last byte if invalid
                }
                len /= 2;

                let mut u8s: Vec<u16> = Vec::new();
                for i in 0..len {
                    let index = 2 * i;
                    let u = util::u8s_to_u16(data_slice[index], data_slice[index + 1]);
                    u8s.push(u);
                }

                return String::from_utf16_lossy(u8s.as_slice());
            }
            &Encoding::Utf32 => {
                let mut len = data.len();
                if len % 4 != 0 {
                    len -= len % 4; // truncate the last byte if invalid
                }
                len /= 4;

                let mut string = String::new();
                for i in 0..len {
                    let index = 4 * i;
                    let u = util::u8s_to_u32(
                        data_slice[index],
                        data_slice[index + 1],
                        data_slice[index + 2],
                        data_slice[index + 3],
                    );
                    match char::from_u32(u) {
                        Some(character) => string.push(character),
                        None => {},
                    }
                }
                string
            }
            _ => String::from_utf8_lossy(data_slice).to_string(), // binaries and other files may contain utf8 data, as well as utf8 itself
        }
    }
}

impl From<Encoding> for String {
    fn from(encoding: Encoding) -> String {
        String::from(match encoding {
            Encoding::Ascii => "ascii",
            Encoding::Binary => "binary",
            Encoding::NoData => "nodata",
            Encoding::Utf16 => "utf16",
            Encoding::Utf32 => "utf32",
            Encoding::Utf8 => "utf8",
        })
    }
}

#[cfg(test)]
mod tests {
    use preprocessors::text::*;

    use binary_object::BinaryObject;
    use util;

    #[test]
    fn test_utf8_extraction() {
        let text = "Dr. Z requires tests, and so do I";
        let utf8_bytes: Vec<u8> = text.bytes().collect();
        let utf8_object = BinaryObject::from(utf8_bytes);
        assert_eq!(
            Encoding::determine_encoding(&utf8_object).extract_text(&utf8_object),
            String::from(text)
        );
    }

    #[test]
    fn test_utf16_extraction() {
        let text = "Dr. Z requires tests, and so do I";
        let utf16_bytes: Vec<u8> = util::hex_to_vec("44 72 2e 20 5a 20 72 65 71 75 69 72 65 73 20 74 65 73 74 73 2c 20 61 6e 64 20 73 6f 20 64 6f 20 49").unwrap();
        let utf16_bytes = BinaryObject::from(utf16_bytes);
        assert_eq!(
            Encoding::determine_encoding(&utf16_bytes).extract_text(&utf16_bytes),
            String::from(text)
        );
    }

    // #[test] // TODO; see https://github.com/milesmcc/ArmorLib/issues/3
    // fn test_utf32_extraction() {
    // }
    //

    #[test]
    fn test_ascii_extraction() {
        let text = "Dr. Z requires tests, and so do I";
        let ascii_bytes: Vec<u8> = util::hex_to_vec("44 72 2e 20 5a 20 72 65 71 75 69 72 65 73 20 74 65 73 74 73 2c 20 61 6e 64 20 73 6f 20 64 6f 20 49").unwrap();
        let ascii_bytes = BinaryObject::from(ascii_bytes);
        assert_eq!(
            Encoding::determine_encoding(&ascii_bytes).extract_text(&ascii_bytes),
            String::from(text)
        );
    }

    #[test]
    fn test_determine_encoding_utf32() {
        // test utf32
        let binary_object_utf32: BinaryObject =
            BinaryObject::from(util::hex_to_vec("FF FE 00 00 00 00 00").unwrap());
        assert_eq!(
            Encoding::determine_encoding(&binary_object_utf32),
            Encoding::Utf32
        );
        assert_eq!(
            String::from(Encoding::determine_encoding(&binary_object_utf32)),
            "utf32"
        );
    }

    #[test]
    fn test_determine_encoding_nodata() {
        // test no data
        let binary_object_no_data: BinaryObject = BinaryObject::from(vec![]);
        assert_eq!(
            Encoding::determine_encoding(&binary_object_no_data),
            Encoding::NoData
        );
        assert_eq!(
            String::from(Encoding::determine_encoding(&binary_object_no_data)),
            "nodata"
        );
    }

    #[test]
    fn test_determine_encoding_ascii() {
        // test ascii
        let binary_object_ascii: BinaryObject =
            BinaryObject::from(util::hex_to_vec("33 33 33 33 33 33 34 32 12 34").unwrap());
        assert_eq!(
            Encoding::determine_encoding(&binary_object_ascii),
            Encoding::Ascii
        );
        assert_eq!(
            String::from(Encoding::determine_encoding(&binary_object_ascii)),
            "ascii"
        );
    }

    #[test]
    fn test_determine_encoding_utf8() {
        // test utf8
        let binary_object_utf8: BinaryObject =
            BinaryObject::from(util::hex_to_vec("EF BB BF 00").unwrap());
        assert_eq!(
            Encoding::determine_encoding(&binary_object_utf8),
            Encoding::Utf8
        );
        assert_eq!(
            String::from(Encoding::determine_encoding(&binary_object_utf8)),
            "utf8"
        );
    }

    #[test]
    fn test_determine_encoding_utf16() {
        // test utf16
        let binary_object_utf16: BinaryObject =
            BinaryObject::from(util::hex_to_vec("FF FE 00 00").unwrap());
        assert_eq!(
            Encoding::determine_encoding(&binary_object_utf16),
            Encoding::Utf16
        );
        assert_eq!(
            String::from(Encoding::determine_encoding(&binary_object_utf16)),
            "utf16"
        );
    }
}
