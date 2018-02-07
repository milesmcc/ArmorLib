use std::collections::HashMap;
use binary_object::BinaryObject;

pub struct ScanObject {
    /// Each root key strinng corresponds to the ID of the preprocessor.
    /// The value of each root pair is a `HashMap` created by the preprocessor.
    /// Refer to each preprocessor's documentation for information about its
    /// respective keys and values.
    metadata: HashMap<String, HashMap<String, String>>,

    filetype: String,

    binary_object: BinaryObject,
}
