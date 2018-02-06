use scan_object::ScanObject;
use std::collections::HashMap;

trait Preprocessor {
    fn process(binary_object: &BinaryObject) -> HashMap<String, String>;
}
