//! This is a module that defines the `ScanReport`. The `ScanReport` is also available in the
//! default namespace (accessible under `armorlib::ScanReport`).

use finding::Finding;
use errors::ArmorlibError;

/// A `ScanReport` details the findings of a `ScanModule`, and includes various 'biographical'
/// information about the scan module.
#[derive(Debug)]
pub struct ScanReport {
    /// The module's name and description.
    pub module_info: (String, String),

    /// The findings of the scan module (see `ArmorLib::Finding`), if applicable. This may be
    /// present regardless of the content of `error`.
    pub findings: Option<Vec<Finding>>,

    /// The error created by the scan module, if applicable. This may be present regardless of the
    /// content of `findings`.
    pub error: Option<ArmorlibError>,
}
