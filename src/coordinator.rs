use scan_result::ScanResult;
use binary_object::BinaryObject;
use errors::ProcessingError;
use std::io::Read;

/// Process the given `BinaryObject` and return a `ScanResult`
pub fn process<T: Read>(binary_object: &BinaryObject) -> Result<ScanResult, ProcessingError> {
    Err(ProcessingError::UnknownProcessingError(String::from(
        "not implemented",
    )))
}
