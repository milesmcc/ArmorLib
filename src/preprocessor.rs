use std::collections::HashMap;

use binary_object::BinaryObject;

pub trait Preprocessor: Sized {
    fn process(binary_object: &BinaryObject) -> HashMap<String, String>;
    fn name() -> &'static str;
}

pub struct PreprocessorObject;

impl Preprocessor for PreprocessorObject {
    fn process(binary_object: &BinaryObject) -> HashMap<String, String> {
        unimplemented!();
    }
    fn name() -> &'static str {
        unimplemented!();
    }
}
