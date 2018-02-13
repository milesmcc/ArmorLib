use scan_object::ScanObject;
use scan_module::ScanModule;
use scan_result::ScanResult;
use scan_report::ScanReport;

// List preprocessors here
pub mod strings;
pub mod unicode_fingerprinting;

pub fn make_default_scan_modules() -> Vec<Box<ScanModule>> {
    vec![
        Box::new(strings::StringsScanModule {}),
        Box::new(unicode_fingerprinting::FingerprintScanModule),
        // ...and add additional preprocessors here
    ]
}

pub fn process(scan_modules: Vec<Box<ScanModule>>, scan_object: &ScanObject) -> ScanResult {
    let mut scan_reports: Vec<ScanReport> = Vec::new();

    for sm in scan_modules {
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
