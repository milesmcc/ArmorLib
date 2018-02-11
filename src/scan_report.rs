use finding::Finding;

#[derive(Debug)]
pub struct ScanReport {
    pub module_name: String,
    pub findings: Vec<Finding>,
}
