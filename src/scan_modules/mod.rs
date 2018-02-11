use binary_object::BinaryObject;
use scan_module::ScanModule;
use scan_result::ScanResult;
use scan_report::ScanReport;

// List preprocessors here
pub mod strings;

pub fn scan_modules() -> Vec<Box<ScanModule>> {
    vec![
        Box::new(strings::StringsScanModule {}),
        // ...and add additional preprocessors here
    ]
}

pub fn process(scan_modules: Vec<Box<ScanModule>>, binary_object: &BinaryObject) -> ScanResult {
    let scan_reports: Vec<ScanReport> = Vec::new();

    for sm in scan_modules {
        // TODO: run all preprocessors
    }

    ScanResult {
        reports: scan_reports,
    }
}
