//! This module defines the ScanModule, a trait that all scan modules must implement in order
//! to be run by ArmorLib. The `ScanModule` trait is available in the root namespace as
//! `armorlib::ScanModule`.

use errors::ArmorlibError;
use scan_object::ScanObject;
use finding::Finding;

/// A trait that defines the necessary functions for `ScanModule`s to implement. To contribute a
/// new scan module, it will also need to be included in `mod.rs` inside the `scan_modules`
/// directory. A scan module is a modular component that finds vulnerabilities to privacy and
/// security inside `ScanObject`s. The `ScanModule` is the core component of ArmorLib.
pub trait ScanModule {
    /// Scan the given `scan_object` and return either a vector of `Finding`s or, in case of an
    /// error, an `ArmorlibError`.
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError>;

    /// Return a `Vec` of `&'static str`s indicating the preprocessors that the scan module relies
    /// on for running. These names should match the name provided by the processor in its `name()`
    /// function.
    fn required_preprocessors(&self) -> Vec<&'static str>;

    /// Returns a tuple containing the name and description of the scan module.
    fn info(&self) -> (&'static str, &'static str);

    /// Returns a `&'static str` of the name of the scan module. This information is drawn from
    /// the information provided by `info()` in its default implementation.
    fn name(&self) -> &'static str {
        return self.info().0;
    }

    /// Returns a `&'static str` of the description of the scan module. This information is drawn
    /// from the information provided by `info()` in its default implementation.
    fn description(&self) -> &'static str {
        return self.info().1;
    }

    /// Returns an `Option<&[&'static str]>` of the filetypes that the scan module is subscribed
    /// to. If a scan module subscribes to a filetype, it will only be run if the data is detected
    /// to be of any of the subscribed filetypes. If a scan module's `subscribed_filetypes()`
    /// function returns `None` (default), it will be run for every filetype.
    ///
    /// These filetypes must match those defined in `preprocessors/filetype`.
    fn subscribed_filetypes(&self) -> Option<&[&'static str]> {
        return None;
    }
}
