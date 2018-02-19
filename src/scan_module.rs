use errors::ArmorlibError;
use scan_object::ScanObject;
use finding::Finding;

pub trait ScanModule {
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError>;
    fn required_preprocessors(&self) -> Vec<&'static str>;
    /// Returns a tuple of the name and description of the scan module.
    fn info(&self) -> (&'static str, &'static str);
    fn name(&self) -> &'static str {
        return self.info().0;
    }
    fn description(&self) -> &'static str {
        return self.info().1;
    }
}
