use std::collections::HashMap;
use binary_object::BinaryObject;

#[derive(Debug)]
pub struct ScanObject {
    metadata: HashMap<String, String>,
    binary_object: BinaryObject,
}
