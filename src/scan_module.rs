use scan_report::ScanReport;
use scan_object::ScanObject;

trait ScanModule {
    fn scan(scan_object: &ScanObject) -> Result<ScanReport, String>;
    fn name() -> &'static str;
}
