use finding::Finding;
use errors::ArmorlibError;

#[derive(Debug)]
pub struct ScanReport {
    /// The module's name and description.
    pub module_info: (String, String),
    pub findings: Option<Vec<Finding>>,
    pub error: Option<ArmorlibError>,
}
