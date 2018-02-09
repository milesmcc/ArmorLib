#[macro_use] use maplit;

use std::collections::HashMap;
use std::u8;

use preprocessor::Preprocessor;
use binary_object::BinaryObject;

struct FiletypePrepreprocessor;

/// Standard filetypes are paired by their magic number and typical file extension.
/// See https://en.wikipedia.org/wiki/List_of_file_signatures for more information.
/// Lowercase filetypes indicate standard file extensions, while uppercase filetypes
/// indicate filetypes that operate without an extension (for example, a Mach-O binary).
fn magic_numbers() -> HashMap<&'static str, &'static str> {
    hashmap!{
        "00 00 01 00" => "ico",
        "42 5A 68" => "bz2",
        "47 49 46 38 37 61" => "gif", // GIF87a
        "47 49 46 38 39 61" => "gif", // GIF89a
        "49 49 2A 00" => "tiff", // little endian
        "4D 4D 00 2A" => "tiff", // big endian
        "FF D8 FF" => "jpg", // this is not technically the spec
        "4D 5A" => "exe",
        "50 4B 03 04" => "zip",
        "50 4B 05 06" => "zip",
        "50 4B 07 08" => "zip",
        "52 61 72 21 1A 07 00" => "rar",
        "52 61 72 21 1A 07 01 00" => "rar",
        "7F 45 4C 46" => "ELF",
        "89 50 4E 47 0D 0A 1A 0A" => "png",
        "CA FE BA BE" => "class",
        "FE ED FA CE" => "MACH-O",
        "FE ED FA CF" => "MACH-O",
        "CE FA ED FE" => "MACH-O",
        "CF FA ED FE" => "MACH-O",
        "25 50 44 46" => "pdf",
        "24 53 44 49 30 30 30 31" => "sdi",
        "4F 67 67 53" => "ogg",
        "38 42 50 53" => "psd",
        "52 49 46 46 ?? ?? ?? ?? 57 41 56 45" => "wav",
        "52 49 46 46 ?? ?? ?? ?? 41 56 49 20" => "avi",
        "FF FB" => "mp3",
        "49 44 33" => "mp3",
        "42 4D" => "bmp",
        "43 44 30 30 31" => "iso",
        "4D 54 68 64" => "mid",
        "D0 CF 11 E0 A1 B1 1A E1" => "doc",
        "43 72 32 34" => "crx",
        "78 01 73 0D 62 62 60" => "dmg",
        "50 4D 4F 43 43 4D 4F 43" => "dat",
        "75 73 74 61 72 00 30 30" => "tar",
        "75 73 74 61 72 20 20 00" => "tar",
        "37 7A BC AF 27 1C" => "7z",
        "1F 8B" => "gz",
        "FD 37 7A 58 5A 00 00" => "xz",
        "04 22 4D 18" => "lz4",
        "77 4F 46 46" => "woff",
        "77 4F 46 32" => "woff2",
        "3C 3F 78 6D 6C 20" => "xml",
        "21 3C 61 72 63 68 3E" => "deb",
        "27 05 19 56" => "uboot",
        "7B 5C 72 74 66 31" => "rtf",
        // TODO: more
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

    pub fn filetype_representation(&self) -> &str {
        match self {
            &Encoding::Ascii => "*ascii",
            &Encoding::Binary => "*binary",
            &Encoding::NoData => "*nodata",
            &Encoding::Utf16 => "*utf16",
            &Encoding::Utf32 => "*utf32",
            &Encoding::Utf8 => "*utf8"
        }
    }
}

fn do_bytes_match(pattern: &str, bytes: &[u8]) -> bool {
    let pattern = String::from(pattern).replace(" ", "");
    let pattern_hex: Vec<&str> = pattern.split(" ").collect();
    if pattern_hex.len() > bytes.len() {
        return false;
    }
    for (index, pat_str) in pattern_hex.iter().enumerate() {
        let pat = String::from(*pat_str);
        if pat == String::from("??") { // TODO: optimize
            continue;
        }
        let byte_str = format!("{:X}", bytes[index]);
        if String::from(byte_str) != pat { // TODO: optimize
            return false;
        }
    }
    true
}

fn determine_file_types(binary_object: &BinaryObject) -> Vec<String> {
    let mut filetypes: Vec<String> = Vec::new();

    // first, we must determine the file's encoding
    let encoding = Encoding::determine_encoding(binary_object);

    // add the encoding filetype to the vec
    filetypes.push(String::from(encoding.filetype_representation()));

    let mut num_bytes = 16;
    if num_bytes > binary_object.data.len() {
        num_bytes = binary_object.data.len();
    }
    let bytes = &binary_object.data.as_slice()[0..num_bytes];

    // match against all
    for (magic_number_str, filetype) in magic_numbers() {
        if do_bytes_match(magic_number_str, bytes) {
            filetypes.push(String::from(filetype));
        }
    }

    return filetypes;
}

impl Preprocessor for FiletypePrepreprocessor {

    /// Returns a map where the keys are file types (without the leading `.`).
    /// Values are not currently used.
    fn process(binary_object: &BinaryObject) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        for filetype in determine_file_types(binary_object).iter() {
            map.insert(filetype.clone(), String::from(""));
        };
        map
    }

    fn name() -> &'static str {
        "filetype"
    }
}

#[cfg(test)]
mod tests {
    use ::preprocessors::filetype::*;
    use ::binary_object::BinaryObject;

    fn hex_to_vec(hex: &str) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for byte in String::from(hex).split(" ") {
            bytes.push(u8::from_str_radix(byte, 16).unwrap());
        }
        bytes
    }

    #[test]
    fn test_determine_encoding() {
        // test utf32
        {
            let binary_object_utf32: BinaryObject = BinaryObject::from(hex_to_vec("FF FE 00 00 00 00 00"));
            assert_eq!(Encoding::determine_encoding(&binary_object_utf32), Encoding::Utf32);
            assert_eq!(Encoding::determine_encoding(&binary_object_utf32).filetype_representation(), "*utf32");
        }

        // test no data
        {
            let binary_object_no_data: BinaryObject = BinaryObject::from(vec![]);
            assert_eq!(Encoding::determine_encoding(&binary_object_no_data), Encoding::NoData);
            assert_eq!(Encoding::determine_encoding(&binary_object_no_data).filetype_representation(), "*nodata");
        }

        // test ascii
        {
            let binary_object_ascii: BinaryObject = BinaryObject::from(hex_to_vec("33 33 33 33 33 33 34 32 12 34"));
            assert_eq!(Encoding::determine_encoding(&binary_object_ascii), Encoding::Ascii);
            assert_eq!(Encoding::determine_encoding(&binary_object_ascii).filetype_representation(), "*ascii");
        }

        // test utf8
        {
            let binary_object_utf8: BinaryObject = BinaryObject::from(hex_to_vec("EF BB BF 00"));
            assert_eq!(Encoding::determine_encoding(&binary_object_utf8), Encoding::Utf8);
            assert_eq!(Encoding::determine_encoding(&binary_object_utf8).filetype_representation(), "*utf8");
        }

        // test utf16
        {
            let binary_object_utf16: BinaryObject = BinaryObject::from(hex_to_vec("FF FE 00 00"));
            assert_eq!(Encoding::determine_encoding(&binary_object_utf16), Encoding::Utf16);
            assert_eq!(Encoding::determine_encoding(&binary_object_utf16).filetype_representation(), "*utf16");
        }
    }

    #[test]
    fn test_filetype_detection() {
        println!("{:?}", determine_file_types(&BinaryObject::from(hex_to_vec(
            "52 61 72 21 1A 07 01 00 23 9B 4B C9 FF E4 FF F1 CF"
        ))));
        // test rar
        assert!(determine_file_types(&BinaryObject::from(hex_to_vec(
            "52 61 72 21 1A 07 01 00 23 9B 4B C9 FF E4 FF F1 CF"
        ))).iter().any(|x| x == &String::from("rar")));
    }
}
