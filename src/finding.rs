use std::fmt;

#[derive(Debug)]
pub struct Finding {
    title: String,
    id: String,
    description: String,
    status: Severity,
    // data: T<DeserializeOwned>
}

enum Severity {
    /// there is no issue with the data
    Ok(String),

    /// the data has a potential problem, but it is not threatening; an afterthought
    Warn(String),

    /// the data has a problem which may compromise security or privacy
    Danger(String),

    /// the data has a problem which will surely compromise security or privacy
    Severe(String)
}

impl fmt::Debug for Severity {
    fn fmt(self: &Severity, formatter: &mut fmt::Formatter) -> Result<String, fmt::Error>{
        match severity {
            &Severity::Ok => Some("Ok"),
            &Severity::Warn => Some("Warn"),
            &Severity::Danger => Some("Danger"),
            &Severity::Severe => Some("Severe"),
            _ => None
        }
    }
}
