use errors::ProcessingError;
use scan_object::ScanObject;
use finding::Finding;

pub trait ScanModule: Sized {
    fn scan(scan_object: &ScanObject) -> Result<Vec<Finding>, ProcessingError>;
    fn name() -> &'static str;
    fn required_preprocessors() -> Vec<&'static str>;
}
