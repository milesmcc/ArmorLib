use std::collections::HashMap;
use std::ops::Index;

use errors::ProcessingError;
use binary_object::BinaryObject;

pub struct ScanObject {
    /// Each root key strinng corresponds to the ID of the preprocessor.
    /// The value of each root pair is a `HashMap` created by the preprocessor.
    /// Refer to each preprocessor's documentation for information about its
    /// respective keys and values.
    pub metadata: HashMap<String, HashMap<String, String>>,

    pub filetype: Option<String>,

    pub binary_object: BinaryObject,
}

// TODO: write a nice test; see https://github.com/milesmcc/ArmorLib/issues/10
impl ScanObject {
    /// Get the given key created by the given preprocessor, where the key and preprocessor are
    /// denoted in the format `<preprocessor/key>`. Will return the HashMap if the
    /// preprocessor is present, ortherwise a `ProcessingError::MissingPreprocessor` error
    /// will be returned.
    pub fn get_metadata(&self, path: &str) -> Result<&String, ProcessingError> {
        let path = String::from(path);
        let key_pair: Vec<&str> = path.split('/').collect();
        let (preprocessor, key) = (
            String::from(*key_pair.index(0_usize)),
            String::from(*key_pair.index(1_usize)),
        );
        match self.metadata.get(&preprocessor) {
            Some(map) => match map.get(&key) {
                Some(value) => Ok(value),
                None => Err(ProcessingError::MissingMetadata(path.clone())),
            },
            None => Err(ProcessingError::MissingPreprocessor(preprocessor)),
        }
    }
}
