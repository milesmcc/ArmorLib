//! Implements a module that attempts to catch a common techinique used to compromise privacy and
//! find sources: Unicode watermarking. There are lots of techniques that are used to do this:
//! right now, this library catches the most common one, using zero-width joiner characters which are
//! invisible to humans but uniquely mark text depending on their locations.

use std::collections::HashMap;

use errors::ArmorlibError;
use scan_module::ScanModule;
use scan_object::ScanObject;
use finding::{Finding, Severity};

/// A scan module that implements checking for Unicode zero-width characters, a common technique used
/// for identifying leakers or otherwise invading privacy.
pub struct UnicodeWatermarkScanModule;

fn watermark_chars() -> HashMap<char, &'static str> {
    hashmap! {
        '\u{200b}' => "Unicode zero-width space: most likely suspicious",
        '\u{feff}' => "Unicode zero-width no-break space: most likely suspicious",
        '\u{200c}' => "Unicode zero-width non-joiner: could be OK, possibly suspicious",
        '\u{200d}' => "Unicode zero-width joiner: could be OK, possibly suspicious",
    }
}

/// Gets the surrounding 20 characters on either side of the given index, truncating if at the end or
/// beginning of the string. Replaces the exact character with "_[HERE]_".
fn surrounding_text(i: usize, text: &str) -> String  {
    let mut surround = String::new();
    let mut is_in_range = false;
    for (j, character) in text.char_indices() {
        // check if in range on left side for first time
        if !is_in_range && i <= 20 + j && i >= j {
            is_in_range = true;
        }
        // check if moving out of range
        if is_in_range && j == 20 + i {
            is_in_range = false;
        }
        // if in range, add to surround
        if is_in_range {
            // replace character itself
            if i == j {
                surround.push_str("_[HERE]_");
            } else {
                surround.push(character);
            }
        }
    }
    surround
}

impl ScanModule for UnicodeWatermarkScanModule {
    /// Returns locations of Unicode zero-width strings.
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
        let mut findings = Vec::new();

        // get text
        let text = scan_object.get_metadata("text/text")?;
        let watermarks = watermark_chars();
        for (i, character) in text.char_indices() {
            match watermarks.get(&character) {
                Some(warn_text) =>  {
                    findings.push(Finding {
                        title: String::from(*warn_text),
                        description: format!("found suspicious character at index {}: \"{}\"",
                                             i, surrounding_text(i, text.as_str())
                        ),
                        id: String::from("UNICODE_WATERMARK"),
                        severity: Severity::Warn(String::from("possible attempt to watermark data")),
                    });
                }
                _ => {}
            }
        }

        Ok(findings)
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("unicode_watermark", "searches for attempts to watermark text using unusual Unicode")
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
    use binary_object::BinaryObject;
    #[test]
    fn test_zero_width_space_char() {
        // has zero-width spaces in it after "the" and "nuclear" (non-breaking and breaking)
        let sus_string = "The﻿ nuclear​ launch codes are 0000, 0001, and 1234.";
        let mut scan_result = process(
            vec![Box::new(UnicodeWatermarkScanModule)],
            Vec::new(),
            BinaryObject::from(sus_string.as_bytes().to_vec()),
            None
        ).unwrap();
        let scan_report = scan_result.reports.pop().unwrap();
        assert_eq!(scan_report.module_info.0.as_str(),
                   "unicode_watermark");
        assert_eq!(scan_report.module_info.1.as_str(),
                   "searches for attempts to watermark text using unusual Unicode");
        let findings = scan_report.findings.unwrap();
        let finding1 = findings.get(0).unwrap();
        let finding2 = findings.get(1).unwrap();
        assert_eq!(finding1.title,
                   "Unicode zero-width no-break space: most likely suspicious");
        assert_eq!(finding2.title,
                   "Unicode zero-width space: most likely suspicious");
        assert_eq!(finding1.id, "UNICODE_WATERMARK");
        assert_eq!(finding1.severity, finding2.severity);
        assert_eq!(finding1.severity, Severity::Warn(String::from("possible attempt to watermark data")));
        assert_eq!(finding1.description,
                   "found suspicious character at index 3: \"The_[HERE]_ nuclear\u{200b} launc\"");
        assert_eq!(finding2.description,
                   "found suspicious character at index 14: \"The\u{feff} nuclear_[HERE]_ launch codes are\"");
    }
    #[test]
    fn test_zero_width_joiner_non_joiner() {
        // has zero-width joiners in it after "the" and "0001" (non-join and join respectively)
        let sus_string = "The‌ nuclear launch codes are 0000, 0001‍, and 1234";
        let mut scan_result = process(
            vec![Box::new(UnicodeWatermarkScanModule)],
            Vec::new(),
            BinaryObject::from(sus_string.as_bytes().to_vec()),
            None
        ).unwrap();
        let scan_report = scan_result.reports.pop().unwrap();
        assert_eq!(scan_report.module_info.0.as_str(),
                   "unicode_watermark");
        assert_eq!(scan_report.module_info.1.as_str(),
                   "searches for attempts to watermark text using unusual Unicode");
        let findings = scan_report.findings.unwrap();
        let finding1 = findings.get(0).unwrap();
        let finding2 = findings.get(1).unwrap();
        assert_eq!(finding1.title,
                   "Unicode zero-width non-joiner: could be OK, possibly suspicious");
        assert_eq!(finding2.title,
                   "Unicode zero-width joiner: could be OK, possibly suspicious");
        assert_eq!(finding1.id, "UNICODE_WATERMARK");
        assert_eq!(finding1.severity, finding2.severity);
        assert_eq!(finding1.severity, Severity::Warn(String::from("possible attempt to watermark data")));
        assert_eq!(finding1.description,
                   "found suspicious character at index 3: \"The_[HERE]_ nuclear launch c\"");
        assert_eq!(finding2.description,
                   "found suspicious character at index 42: \"codes are 0000, 0001_[HERE]_, and 1234\"");
    }
}
