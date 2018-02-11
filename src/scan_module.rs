use errors::ProcessingError;
use scan_object::ScanObject;
use finding::Finding;

pub trait ScanModule {
    fn scan(scan_object: &ScanObject) -> Result<Vec<Finding>, ProcessingError>
    where
        Self: Sized;
    fn name() -> &'static str
    where
        Self: Sized;
    fn required_preprocessors() -> Vec<&'static str>
    where
        Self: Sized;
}
