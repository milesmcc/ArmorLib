//! This module defines the `process` trait, which is designed to make processing simple
//! structs, like a

use coordinator;
use scan_result::ScanResult;
use scan_modules::make_default_scan_modules;
use errors::ArmorlibError;
use binary_object::BinaryObject;
use std::fs::File;

/// A trait that allows for the object to be run through the ArmorLib system with only a single
/// call to `.process()`. This is a perfectly valid way of passing objects through ArmorLib,
/// however `coordinator::Process` is also available for more granular control.
///
/// # Examples
///
/// ```rust
/// use armorlib::Process;
/// let data: Vec<u8> = vec![]; // empty data for demo
/// let scan_result = data.process().unwrap();
/// ```
pub trait Process {
    fn process(self) -> Result<ScanResult, ArmorlibError>
    where
        Self: Sized,
        BinaryObject: From<Self>,
    {
        coordinator::process(
            make_default_scan_modules(),
            Vec::new(),
            BinaryObject::from(self),
            None,
        )
    }
}

/// An empty implementation of Process for `Vec<u8>`. Because `BinaryObject: From<Vec<u8>>` is
/// implemented in the `binary_object` module, no special implementation is necessary here.
/// Provided that `binary_object` is in scope, you can just call `vec.process()`.
///
/// # Examples
///
/// ```rust
/// use armorlib::binary_object::BinaryObject;
/// use armorlib::process::Process;
/// let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
/// let _scan_result = vec.process().unwrap(); // this is a `ScanResult` object
/// ```
impl Process for Vec<u8> {}

// TODO: documentation. How do you create fake demo files to work with?
impl Process for File {}

/// An empty implementation of Process for `BinaryObject`. Because `BinaryObject:
/// From<BinaryObject>>` is implemented in the `binary_object` module, no special implementation
/// is necessary here. Provided that `binary_object` is in scope, you can just call
/// `binary_object.process()`.
///
/// # Examples
///
/// ```rust
/// use armorlib::binary_object::BinaryObject;
/// use armorlib::process::Process;
/// let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
/// let bin_obj = BinaryObject::from(vec);
/// let _scan_result = bin_obj.process().unwrap(); // this is a `ScanResult` object
/// ```
impl Process for BinaryObject {}

#[cfg(test)]
mod tests {
    use super::*;
    use binary_object::BinaryObject;

    #[test]
    fn test_for_vec() {
        let data: Vec<u8> = vec![]; // empty data for demo
        let _scan_result = data.process().unwrap(); // if unwrap fails, so will test
    }

    #[test]
    fn test_for_binary_object() {
        let binary_object = BinaryObject::from(vec![]); // empty data for demo
        let _scan_result = binary_object.process().unwrap(); // if unwrap fails, so will test
    }

    // TODO: test for file
}
