use std::collections::HashMap;

use binary_object::BinaryObject;
use preprocessor::Preprocessor;

// List preprocessors here
pub mod filetype;
pub mod text;
pub mod hex;

pub fn make_default_preprocessors() -> Vec<Box<Preprocessor>> {
    vec![
        Box::new(filetype::FiletypePrepreprocessor {}),
        Box::new(text::TextPreprocessor {}),
        Box::new(hex::HexPreprocessor {}),
        // ...and add additional default preprocessors here
    ]
}

pub fn process(
    preprocessors: Vec<Box<Preprocessor>>,
    binary_object: &BinaryObject,
) -> HashMap<String, HashMap<String, String>> {
    let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();

    for preprocessor_box in preprocessors.iter() {
        let results = preprocessor_box.process(binary_object);
        map.insert(String::from(preprocessor_box.name()), results);
        // TODO: proper error checking above
    }

    map
}
