#[derive(Debug)]
pub struct Finding {
    title: String,
    id: String,
    description: String,
    status: Severity
}

#[derive(Debug)]
pub enum Severity {
    /// there is no issue with the data
    Ok(String),

    /// the data has a potential problem, but it is not threatening; an afterthought
    Warn(String),

    /// the data has a problem which may compromise security or privacy
    Danger(String),

    /// the data has a problem which will surely compromise security or privacy
    Severe(String)
}
