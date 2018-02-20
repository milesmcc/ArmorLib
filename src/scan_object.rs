//! This module defines the ScanObject, which represents a `BinaryObject` that has been
//! preprocessed. The `ScanObject` struct defined in this module is accessible in the root
//! namespace as `armorlib::ScanObject`.

use std::collections::HashMap;
use std::ops::Index;

use errors::ArmorlibError;
use binary_object::BinaryObject;

/// A struct that represents a `BinaryObject` (with associated `filetype`) that has been
/// preprocessed and therefore contains `metadata`. This is the object that is passed to
/// `ScanModule`s.
///
/// This object is typically only created by the `coordinator`, and there are few cases
/// in which it would be necessary to create this object in your own library. Still, it is
/// public so that future versions of the library might be more extendable.
pub struct ScanObject {
    /// A `HashMap` that represents data created by the preprocessors. Each root key string
    /// corresponds to the ID of the preprocessor. The value of each root pair is a `HashMap`
    /// created by the preprocessor. Refer to each preprocessor's documentation for information
    /// about its respective keys and values.
    pub metadata: HashMap<String, HashMap<String, String>>,

    /// The filetype of the data. This is passed in by the user, and is not determined by any
    /// preprocessor. Instead, it is extracted by the CLI or provided by the bound
    /// program. It should not be trusted; a file may advertise itself with its extension as a
    /// `.pdf` file but may in fact be a `.tiff` file, for example.
    ///
    /// Because there is no guarantee that any filetype will be given by the user (for example,
    /// when all that is passed to form the root `BinaryObject` is a `Vec<u8>`), there is a
    /// possibility of absence.
    pub filetype: Option<String>,

    /// The BinaryObject that the ScanObject contains.
    pub binary_object: BinaryObject,
}

impl ScanObject {
    /// Get the given key created by the given preprocessor, where the key and preprocessor are
    /// denoted in the format `<preprocessor/key>`. Will return the HashMap if the
    /// preprocessor is present, ortherwise a `ProcessingError::MissingPreprocessor` error
    /// will be returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate armorlib;
    /// # use std::collections::HashMap;
    /// use armorlib::{ScanObject, ArmorlibError, BinaryObject};
    /// #[macro_use]
    /// extern crate maplit;
    /// # fn main() {
    /// # let metadata: HashMap<String, HashMap<String, String>> = hashmap!{
    /// #     String::from("preprocessor") => hashmap!{
    /// #         String::from("key_name") => String::from("value"),
    /// #     }
    /// # };
    /// # let scan_object = ScanObject {
    /// #     metadata: metadata,
    /// #     filetype: None,
    /// #     binary_object: BinaryObject::from(vec![]),
    /// # };
    /// // given the scan object with the name `scan_object`, we can easily access its metadata:
    /// assert_eq!(
    ///     // use a `match` statement in your own code for safety
    ///     scan_object.get_metadata("preprocessor/key_name").unwrap(),
    ///     &String::from("value")
    /// );
    ///
    /// // if we try to access metadata from a preprocessor that doesn't exist,
    /// // we'll get a `MissingPreprocessor` error.
    /// assert_eq!(
    ///     scan_object.get_metadata("fake_preprocessor/key_name"),
    ///     Err(ArmorlibError::MissingPreprocessor(String::from(
    ///         "fake_preprocessor"
    ///     )))
    /// );
    ///
    /// // if we try to access metadata that doesn't exist from a preprocessor that does, we'll
    /// // get a `MissingMetadata` error.
    /// assert_eq!(
    ///     scan_object.get_metadata("preprocessor/fake_key"),
    ///     Err(ArmorlibError::MissingMetadata(String::from("preprocessor/fake_key")))
    /// );
    /// # }
    /// ```
    pub fn get_metadata(&self, path: &str) -> Result<&String, ArmorlibError> {
        let path = String::from(path);
        let key_pair: Vec<&str> = path.split('/').collect();
        let (preprocessor, key) = (
            String::from(*key_pair.index(0_usize)),
            String::from(*key_pair.index(1_usize)),
        );
        match self.metadata.get(&preprocessor) {
            Some(map) => match map.get(&key) {
                Some(value) => Ok(value),
                None => Err(ArmorlibError::MissingMetadata(path.clone())),
            },
            None => Err(ArmorlibError::MissingPreprocessor(preprocessor)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_metadata() {
        // in this test, 'modules' refer truly to preprocessors.
        let metadata: HashMap<String, HashMap<String, String>> = hashmap!{
            String::from("module1") => hashmap!{
                String::from("key") => String::from("m1::value"),
                String::from("key2") => String::from("m1::value2")
            },
            String::from("module2") => hashmap!{
                String::from("key3") => String::from("m2::value3"),
                String::from("key2") => String::from("m2::value2")
            }
        };
        let scan_object = ScanObject {
            metadata: metadata,
            filetype: None,
            binary_object: BinaryObject::from(vec![]),
        };
        assert_eq!(
            scan_object.get_metadata("module1/key").unwrap(),
            &String::from("m1::value")
        );
        assert_eq!(
            scan_object.get_metadata("module1/key2").unwrap(),
            &String::from("m1::value2")
        );
        assert_eq!(
            scan_object.get_metadata("module2/key3").unwrap(),
            &String::from("m2::value3")
        );
        assert_eq!(
            scan_object.get_metadata("module2/key2").unwrap(),
            &String::from("m2::value2")
        );
        assert_eq!(
            scan_object.get_metadata("fakemodule/key2"),
            Err(ArmorlibError::MissingPreprocessor(String::from(
                "fakemodule"
            )))
        );
        assert_eq!(
            scan_object.get_metadata("module2/key4"),
            Err(ArmorlibError::MissingMetadata(String::from("module2/key4")))
        );
    }
}
