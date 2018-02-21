//! This module defines the `Finding`, a struct that represents a vulnerability found by a
//! `ScanModule`.

/// A struct that represents a vulernability found by a `ScanModule`.
#[derive(Debug)]
pub struct Finding {
    /// The title of the vulenrability. This is human-meaningful, and should make sense on its
    /// own with only the context of the scan module's name. This shouldn't be a complete sentence;
    /// the knowledge of the scan module should make only a noun necessary; that is, the title
    /// indicates what the scan module actually found. Therefore, it is not necessary to include
    /// a verb.
    ///
    /// For example, the title of a `Finding` created by the `strings` scan module might be
    /// `XOR DECODE LOOP (COMMON TO MICROSOFT MALWARE)`. Because this will be listed explicitly
    /// as a finding _of_ the Strings scan module, such a title will make sense to the user.
    pub title: String,

    /// A consistent and unique identifier for the type of finding in screaming snake case.
    /// All findings of the same type should have the same ID. For example, the ID of a finding
    /// of the `string` scan module might be `SUSPICIOUS_STRING`.
    pub id: String,

    /// A human meaningful description of the finding. This should be between one and two
    /// full sentences.
    pub description: String,

    /// The `Severity` of the finding. This ranges from `Ok` to `Severe`. See the documentation
    /// for `Severity` for more in-depth explanations of each possible state.
    pub severity: Severity,
}

/// An enum that represents severity. While not necessarily limited to findings, they are commonly
/// used in the `Finding` struct to designate the severity of a finding related to a privacy
/// or security vulnerability. The following documentation assumes this use case.
///
/// The attached `String` is a description of the potential ramifications of the vulnerability.
/// For example, this explanation string may be `likely indicates malicious software` or,
/// alternatively, `not an issue`. This should not be a full sentence.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Severity {
    /// For use there is no issue.
    Ok(String),

    /// For use when the data has a potential problem, but it is not threatening; an afterthought.
    Warn(String),

    /// For use when the data has a problem which _may_ compromise security or privacy.
    Danger(String),

    /// For use then the data has a problem which will _surely_ compromise security or privacy.
    Severe(String),
}
