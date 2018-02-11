use scan_result::ScanResult;
use binary_object::BinaryObject;
use errors::ProcessingError;
use preprocessor::Preprocessor;
use scan_module::ScanModule;
use preprocessors;
use scan_modules;

/// Process the given `BinaryObject` and return a `ScanResult`
pub fn process(binary_object: &BinaryObject) -> Result<ScanResult, ProcessingError> {
    unimplemented!();
}
