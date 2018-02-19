use coordinator;
use scan_result::ScanResult;
use scan_modules::make_default_scan_modules;
use errors::ArmorlibError;
use binary_object::BinaryObject;
use std::fs::File;
use std::io::Read;

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
pub trait Process: Sized {
    fn process(self) -> Result<ScanResult, ArmorlibError>
    where
        Self: Sized,
    {
        let binary_object_data = self.to_binary_object()?;
        coordinator::process(
            make_default_scan_modules(),
            Vec::new(),
            binary_object_data,
            None,
        )
    }

    fn to_binary_object(self) -> Result<BinaryObject, ArmorlibError>;
}

/// An implementation of `Process` for `Vec<u8>`, so that `vec.process()` can be called when
/// `Process` is in scope.
///
/// # Examples
///
/// ```rust
/// use armorlib::Process;
/// let data: Vec<u8> = vec![]; // empty data for demo
/// let scan_result = data.process().unwrap();
/// ```
impl Process for Vec<u8> {
    fn to_binary_object(self) -> Result<BinaryObject, ArmorlibError> {
        Ok(BinaryObject::from(self))
    }
}

/// An implementation of `Process` for `std::io::File`, so that `file.process()` can be called when
/// `Process` is in scope.
impl Process for File {
    fn to_binary_object(mut self) -> Result<BinaryObject, ArmorlibError> {
        let mut data_vec: Vec<u8> = Vec::new();
        match self.read_to_end(&mut data_vec) {
            Ok(_length) => Ok(BinaryObject::from(data_vec)),
            Err(error) => Err(ArmorlibError::ReadFileError(format!(
                "unable to read the file {:?}: {}",
                self, error
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_vec() {
        let file: Vec<u8> = vec![]; // empty data for demo
        let _scan_result = file.process().unwrap(); // if unwrap fails, so will test
    }
}
