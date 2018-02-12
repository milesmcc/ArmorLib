use std::collections::HashMap;

use binary_object::BinaryObject;

pub trait Preprocessor {
    fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String>;
    fn name(&self) -> &'static str;
}
