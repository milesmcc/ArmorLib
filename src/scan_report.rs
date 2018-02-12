use finding::Finding;
use errors::ProcessingError;

#[derive(Debug)]
pub struct ScanReport {
    pub module_name: String,
    pub findings: Option<Vec<Finding>>,
    pub error: Option<ProcessingError>,
}
