use serde::de::DeserializeOwned;
use std::fmt;

enum Severity {
    /// there is no issue with the data
    Ok,

    /// the data has a potential problem, but it is not threatening; an afterthought
    Warn,

    /// the data has a problem which may compromise security or privacy
    Danger,

    /// the data has a problem which will surely compromise security or privacy
    Severe
}

impl fmt::Debug for Severity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        match self {
            Ok => Some("Ok"),
            Warn => Some("Warn"),
            Danger => Some("Danger"),
            Severe => Some("Severe"),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Finding {
    title: String,
    id: String,
    description: String,
    status: Severity,
    // data: T<DeserializeOwned>
}

#[derive(Debug)]
pub struct ScanReport {
    module_name: String,
    findings: Vec<Finding>
}
