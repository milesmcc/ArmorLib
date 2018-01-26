use std::collections::HashMap;
use binary_object::BinaryObject;

pub struct ScanObject {
    metadata: HashMap<String, String>,
    binary_object: BinaryObject,
}
