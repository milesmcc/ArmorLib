use serde::de::DeserializeOwned;
use std::fmt;
use finding;

#[derive(Debug)]
pub struct ScanReport {
    module_name: String,
    findings: Vec<finding::Finding>
}
