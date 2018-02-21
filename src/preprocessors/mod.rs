//! This module defines and manages the default preprocessors available to ArmorLib.
//!
//! To contribute a new `Preprocessor`, you must:
//!     1. publicly import it into this module via `pub mod`.
//!     2. instantiate it in `make_default_preprocessors()`.

use std::collections::HashMap;

use binary_object::BinaryObject;
use preprocessor::Preprocessor;

// List preprocessors here
pub mod filetype;
pub mod text;
pub mod hex;

/// Create a `Vec<Box<Preprocessor>>` of the core preprocessors available to ArmorLib. This will
/// instantiate the preprocessors.
///
/// # Examples
///
/// ```rust
/// use armorlib::preprocessors;
/// let all_default_preprocessors = preprocessors::make_default_preprocessors();
/// ```
pub fn make_default_preprocessors() -> Vec<Box<Preprocessor>> {
    vec![
        Box::new(filetype::FiletypePrepreprocessor {}),
        Box::new(text::TextPreprocessor {}),
        Box::new(hex::HexPreprocessor {}),
        // ...and add additional default preprocessors here
    ]
}

/// Process the given `Vec<Box<Preprocessor>>` on the given `&BinaryObject` and return a
/// `HashMap<String, HashMap<String>>`. While concurrency is not yet available in ArmorLib, it
/// will be implemented in this function, if anywhere.
///
/// In _nearly all_ cases, it is better to perform `File.process()`, `Vec<u8>.process()`, or even
/// `coordinator::process()` than to use this function. The previous functions will make sure
/// that everything is set up properly; using this function alone will require you to manage
/// the preprocessors and later scanning yourself. If you need fine grained control, use this
/// function. Otherwise, don't—you'll save yourself a headache.
///
/// # Arguments
/// * `preprocessors`: a `Vec<Box<preprocessors>>` of the preprocessors to be run.
/// * `binary_object`: a reference to a `BinaryObject` on which the preprocessors will run.
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
