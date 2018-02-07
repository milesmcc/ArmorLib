use finding::Finding;
use serde;

#[derive(Debug)]
pub struct ScanReport {
    module_name: String,
    findings: Vec<Finding>
}
