extern crate serde_json;

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

pub struct Finding {
    title: &str,
    id: &str,
    description: &str,
    status: Severity,
    data: Encodable
}

pub struct ScanReport {
    module_name: &str,
    findings: Vec<Finding>
}
