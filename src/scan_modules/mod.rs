use scan_object::ScanObject;
use scan_module::ScanModule;
use scan_result::ScanResult;
use scan_report::ScanReport;

// List preprocessors here
pub mod strings;

pub fn make_default_scan_modules() -> Vec<Box<ScanModule>> {
    vec![
        Box::new(strings::StringsScanModule {}),
        // ...and add additional preprocessors here
    ]
}

pub fn process(scan_modules: Vec<Box<ScanModule>>, scan_object: &ScanObject) -> ScanResult {
    let mut scan_reports: Vec<ScanReport> = Vec::new();

    for sm in scan_modules {
        let report: ScanReport = match sm.scan(scan_object) {
            Ok(findings) => ScanReport {
                error: None,
                findings: Some(findings),
                module_name: String::from(sm.name()),
            },
            Err(error) => ScanReport {
                error: Some(error),
                findings: None,
                module_name: String::from(sm.name()),
            }
        };
        scan_reports.push(report);
    }

    ScanResult {
        reports: scan_reports,
    }
}
