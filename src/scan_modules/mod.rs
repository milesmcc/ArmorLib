//! This module defines and manages the default scan modules available to ArmorLib.
//!
//! To contribute a new `ScanModule`, you must:
//!     1. publicly import it into this module via `pub mod`.
//!     2. instantiate it in `make_default_scan_modules()`.

use scan_object::ScanObject;
use scan_module::ScanModule;
use scan_result::ScanResult;
use scan_report::ScanReport;

// List preprocessors here
pub mod strings;
pub mod unicode_watermark;
pub mod exif;

/// Create a `Vec<Box<ScanModule>>` of the core scan modules available to ArmorLib. This will
/// instantiate the scan modules.
///
/// # Examples
///
/// ```rust
/// use armorlib::scan_modules;
/// let all_default_scan_modules = scan_modules::make_default_scan_modules();
/// ```
pub fn make_default_scan_modules() -> Vec<Box<ScanModule>> {
    vec![
        Box::new(strings::StringsScanModule {}),
        Box::new(unicode_watermark::UnicodeWatermarkScanModule {}),
        Box::new(exif::ExifScanModule {}),
        // ...and add additional preprocessors here
    ]
}

/// Process the given `Vec<Box<ScanModule>>` on the given `ScanObject` and return a ScanResult.
/// While concurrency is not yet available in ArmorLib, it will be implemented in this function,
/// if anywhere. Be sure that the given `ScanObject` has the necessary metadata for the
/// scan modules; no preprocessor checking is done here.
///
/// In _nearly all_ cases, it is better to perform `File.process()`, `Vec<u8>.process()`, or even
/// `coordinator::process()` than to use this function. The previous functions will make sure
/// that everything is set up properly; using this function alone will require you to manage
/// the preprocessors yourself.
///
/// # Arguments
/// * `scan_modules`: a `Vec<Box<ScanModule>>` of the scan modules to be run.
/// * `scan_object`: a reference to a `ScanObject` with all necessary metadata present.
pub fn process(scan_modules: Vec<Box<ScanModule>>, scan_object: &ScanObject) -> ScanResult {
    let mut scan_reports: Vec<ScanReport> = Vec::new();

    let mut skipped_scan_modules: Vec<&'static str> = Vec::new();

    for sm in scan_modules {
        // do not run scan modules that have not subscribed to the filetype
        if sm.subscribed_filetypes().is_some() {
            let filetypes: Vec<String> = sm.subscribed_filetypes()
                .unwrap()
                .iter()
                .map(|f| String::from(*f))
                .collect();
            let mut found = false;
            if &scan_object.filetype.is_some() == &true {
                let given_filetype: &String = match &scan_object.filetype.as_ref() {
                    &Some(filetype) => &filetype,
                    &None => unreachable!(),
                };
                if filetypes.contains(&given_filetype) {
                    found = true;
                }
            }
            for filetype in scan_object.detected_filetypes.as_slice() {
                if filetypes.contains(&filetype) {
                    found = true;
                }
            }

            if !found {
                skipped_scan_modules.push(sm.name());
                continue;
            }
        }

        let report: ScanReport = match sm.scan(scan_object) {
            Ok(findings) => ScanReport {
                error: None,
                findings: match findings.len() {
                    0 => None,
                    _ => Some(findings),
                },
                module_info: (String::from(sm.name()), String::from(sm.description())),
            },
            Err(error) => ScanReport {
                error: Some(error),
                findings: None,
                module_info: (String::from(sm.name()), String::from(sm.description())),
            },
        };
        scan_reports.push(report);
    }

    ScanResult {
        reports: scan_reports,
    }
}
