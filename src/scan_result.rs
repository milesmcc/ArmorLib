//! This is a module that defines the `ScanResult`, the "final product" when ArmorLib processes a
//! `BinaryObject`. It is a collection (`Vec`) of `ScanReport`s, each of which was created by a
//! `ScanModule`. `ScanResult` is also available under the default namespace as
//! `armorlib::ScanResult`.

use scan_report::ScanReport;

/// A struct that represents a scan result. It is a wrapper for `ScanReport`s created by
/// `ScanModule`s.
#[derive(Debug)]
pub struct ScanResult {
    pub reports: Vec<ScanReport>,
}
