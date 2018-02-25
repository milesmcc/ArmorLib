//! ArmorLib is a library that allows for any binary object—be it a `File` or a `Vec<u8>`—to be
//! scanned for threats to security and privacy. It does this by offering several modular
//! 'scan modules' that each search for a specific type of vulnerability.
//!
//! For information on how to use ArmorLib, see [armorlib.org](https://armorlib.org), or browse
//! this `rustdoc`.

#[macro_use]
extern crate maplit;
extern crate aho_corasick;
extern crate crypto;
extern crate exif;

pub mod coordinator;
pub mod binary_object;
pub mod scan_module;
pub mod scan_object;
pub mod scan_report;
pub mod scan_result;
pub mod finding;
pub mod errors;
pub mod preprocessor;
pub mod process;
pub mod util;

pub mod preprocessors;
pub mod scan_modules;

pub use process::Process;
pub use binary_object::BinaryObject;
pub use finding::Finding;
pub use preprocessor::Preprocessor;
pub use scan_module::ScanModule;
pub use scan_object::ScanObject;
pub use scan_report::ScanReport;
pub use scan_result::ScanResult;
pub use errors::ArmorlibError;
