/// This scan module searches for common problematic strings.
/// It uses the Aho–Corasick algorithm for `n*log(s) search time complexity, where `n`
/// is the length of the string and s is the number of strings in the string database.

use std::collections::HashMap;

use aho_corasick::{AcAutomaton, Automaton};

use errors::ProcessingError;
use scan_module::ScanModule;
use scan_object::ScanObject;
use finding::{Finding, Severity};

pub struct StringsScanModule;

fn suspicious_strings() -> HashMap<&'static str, &'static str> {
    // See https://github.com/Yara-Rules/rules
    hashmap!{
        "48 8B CD E8 60 FF FF FF 48 FF C3 32 44 1E FF 48 FF CF 88 43 FF" =>
            "xor decode loop for <PolishBankRAT_srservice>",
    }
}

impl ScanModule for StringsScanModule {
    fn scan(scan_object: &ScanObject) -> Result<Vec<Finding>, ProcessingError> {
        let mut findings: Vec<Finding> = Vec::new();

        let strings = suspicious_strings();
        let aut: AcAutomaton<&str> = strings.keys().cloned().collect();
        let hex = scan_object.get_metadata("hex/hex_data")?;
        let matches = aut.find(&hex);

        for item in matches {
            let pattern: &str = aut.pattern(item.pati);
            let description: &str = match strings.get(pattern) {
                Some(description) => description,
                None => {
                    return Err(ProcessingError::UnknownProcessingError(String::from(
                        "matched key not found",
                    )))
                }
            };
            findings.push(Finding {
                title: String::from(description),
                description: format!("found suspicious string: {}", pattern),
                id: String::from("SUSPICIOUS_STRING"),
                severity: Severity::Warn(String::from("may indicate malware"))
            })
        }

        Ok(findings)
    }

    fn name() -> &'static str {
        "strings"
    }

    fn required_preprocessors() -> Vec<&'static str> {
        vec!["hex"]
    }
}

// TODO: tests
