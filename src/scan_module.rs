use scan_report::ScanReport;
use scan_object::ScanObject;

trait ScanModule {
    fn process(scan_object: &ScanObject) -> Result<ScanReport, String>;
}
