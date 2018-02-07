use finding::Finding;

#[derive(Debug)]
pub struct ScanReport {
    module_name: String,
    findings: Vec<Finding>
}
