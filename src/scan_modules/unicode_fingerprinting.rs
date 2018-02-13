//! Implements a module that attempts to catch a common techinique used to compromise privacy and
//! find sources: Unicode fingerprinting. There are lots of techniques that are used to do this:
//! right now, this library catches the most common one, using zero-width joiner characters which are
//! invisible to humans but uniquely mark text depending on their locations.

use std::collections::HashMap;

use errors::ProcessingError;
use scan_module::ScanModule;
use scan_object::ScanObject;
use finding::{Finding, Severity};

/// A scan module that implements checking for Unicode zero-width characters, a common technique used
/// for identifying leakers or otherwise invading privacy.
pub struct FingerprintScanModule;

fn fingerprint_chars() -> HashMap<char, &'static str> {
    hashmap! {
        '\u{200b}' => "Unicode zero-width space: most likely suspicious",
        '\u{feff}' => "Unicode zero-width no-break space: most likely suspicious",
    }
}

impl ScanModule for FingerprintScanModule {
    /// Returns locations of Unicode zero-width strings.
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ProcessingError> {
        let mut findings = Vec::new();
        
        // get text
        let text = scan_object.get_metadata("text/text")?;
        let chars = text.chars();
        let fingerprints = fingerprint_chars();
        for (i, character) in chars.enumerate() {
            match fingerprints.get(&character) {
                Some(warn_text) =>  {
                    findings.push(Finding {
                        title: String::from(*warn_text),
                        description: format!("Found suspicious character at index {}", i),
                        id: String::from("UNICODE_FINGERPRINT"),
                        severity: Severity::Warn(String::from("possible attempt to fingerprint data")),
                    });                    
                }
                _ => {}
            }
        }

        Ok(findings)
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("unicode_fingerprinting", "searches for attempts to fingerprint text using unusual Unicode")
    }

    fn required_preprocessors(&self) -> Vec<&'static str> {
        vec!["text"]
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use coordinator::process;
    use scan_modules::make_default_scan_modules;
    use binary_object::BinaryObject;
    #[test]
    fn test_zero_width_space_char() {
        // has zero-width spaces in it after "the" and "nuclear" (non-breaking and breaking)
        let sus_string = "The﻿ nuclear​ launch codes are 0000, 0001, and 1234.";
        let mut scan_result = process(
            vec![Box::new(FingerprintScanModule)],
            Vec::new(),
            BinaryObject::from(sus_string.as_bytes().to_vec()),
            None
        ).unwrap();
        let scan_report = scan_result.reports.pop().unwrap();
        assert_eq!(scan_report.module_info.0.as_str(),
                   "unicode_fingerprinting");
        assert_eq!(scan_report.module_info.1.as_str(),
                   "searches for attempts to fingerprint text using unusual Unicode");
        let findings = scan_report.findings.unwrap();
        let finding1 = findings.get(0).unwrap();
        let finding2 = findings.get(1).unwrap();
        assert_eq!(finding1.title,
                   "Unicode zero-width no-break space: most likely suspicious");
        assert_eq!(finding2.title,
                   "Unicode zero-width space: most likely suspicious");
        assert_eq!(finding1.id, "UNICODE_FINGERPRINT");
        assert_eq!(finding1.severity, finding2.severity);
        assert_eq!(finding1.severity, Severity::Warn(String::from("possible attempt to fingerprint data")));
        assert_eq!(finding1.description,
                   "Found suspicious character at index 3");
        assert_eq!(finding2.description,
                   "Found suspicious character at index 12");        
    }
}
