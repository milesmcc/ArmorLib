/// This preprocessor converts the given binary into a space-separated hex string under the
/// key 'hex_data'. While this is not a complicated task, the hex preprocessor exists to remove
/// unnnecessary duplicate code and memory use, as searching the hex string is a common
/// function.
use std::collections::HashMap;
use std::ops::Add;

use preprocessor::Preprocessor;
use binary_object::BinaryObject;

pub struct HexPreprocessor;

impl Preprocessor for HexPreprocessor {
    fn process(binary_object: &BinaryObject) -> HashMap<String, String> {
        let mut string = String::new();
        for byte in &binary_object.data {
            string.push_str(&format!("{:02X} ", byte));
        }
        hashmap! {
            String::from("hex_data") => string
        }
    }

    fn name() -> &'static str {
        "hex"
    }
}

// TODO: tests
