use std::collections::HashMap;

use binary_object::BinaryObject;

pub trait Preprocessor {
    fn process(binary_object: &BinaryObject) -> HashMap<String, String> where Self:Sized;
    fn name() -> &'static str where Self:Sized;
}
