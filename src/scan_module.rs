use errors::ProcessingError;
use scan_object::ScanObject;
use finding::Finding;

pub trait ScanModule {
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ProcessingError>;
    fn name(&self) -> &'static str;
    fn required_preprocessors(&self) -> Vec<&'static str>;
}
