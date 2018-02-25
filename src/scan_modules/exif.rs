//! This scan module searches for EXIF metadata in JPEG and TIFF files. EXIF metadata often
//! contains sensitive private information (like location), and poses a serious threat to
//! privacy and security.

use std::io::BufReader;
use exif;
use exif::{Error, Tag, Value};

use errors::ArmorlibError;
use scan_module::ScanModule;
use scan_object::ScanObject;
use finding::{Finding, Severity};

pub struct ExifScanModule;

impl ScanModule for ExifScanModule {
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
        // check if filetype can even have EXIF, and if not, return an empty vec
        if scan_object.get_metadata("filtype/tiff").is_err()
            && scan_object.get_metadata("filetype/jpg").is_err()
        {
            return Ok(vec![]);
        }

        // read EXIF metadata
        let reader = match exif::Reader::new(&mut BufReader::new(&*scan_object.binary_object.data))
        {
            Ok(reader) => reader,
            Err(error) => match error {
                Error::NotFound("No Exif data found") => return Ok(vec![]),
                _ => return Err(ArmorlibError::ParseError(format!("{:?}", error))),
            },
        };

        let fields = reader.fields();

        let mut findings: Vec<Finding> = Vec::new();

        // iterate through each field and create a Finding
        for field in fields {
            let tag: &Tag = &field.tag;
            let value: &Value = &field.value;

            if tag.default_value().is_some() {
                if format!("{}", value.display_as(*tag))
                    == format!("{}", tag.default_value().unwrap().display_as(*tag))
                {
                    println!("default value found");
                    continue; // has default value, no need to worry
                }
            }

            let description = match tag.description() {
                Some(description) => description,
                None => "<unknown tag>",
            };

            let level = match tag.number() {
                271 | 272 | 305 => Severity::Danger(format!( // make, model, software
                    "'{}' tag can be used to identify origin device",
                    description
                )),
                306 => Severity::Danger(format!( // datetime
                    "'{}' tag can be used to track creator",
                    description
                )),
                315 | 42032 | 42033 | 42034 => {
                    Severity::Danger(format!( // artist, cameraownername, serial number,
                    "'{}' tag can be used to identify creator",
                    description
                ))
                }
                36867 | 36868 => Severity::Danger(format!( // datetimeoriginal
                    "'{}' tag can be used to track creator",
                    description
                )),
                36880 | 36881 | 36882 => Severity::Danger(format!( // offsettime
                    "'{}' tag can be used to determine creator's region",
                    description
                )),
                37888 | 37889 | 37890 | 37891 => {
                    Severity::Danger(format!( // temp, humidity, pressure, water depth
                    "'{}' tag can be used to determine creator's general location",
                    description
                ))
                }
                41492 | 0...31 => Severity::Danger(format!( // subject location, GPS info
                    "'{}' tag can be used to determine creator's location",
                    description
                )),
                _ => Severity::Warn(format!( // default
                    "'{}' tag could reveal private information",
                    description
                )),
            };

            let finding = Finding {
                title: format!("'{}' EXIF metadata detected", description),
                id: String::from("EXIF_METADATA"),
                description: format!(
                    "The '{}' tag ({}) was found in the EXIF metadata: {}",
                    description,
                    tag.number(),
                    value.display_as(*tag)
                ),
                severity: level,
            };

            findings.push(finding);
        }

        Ok(findings)
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("exif", "detects dangerous EXIF metadata")
    }

    fn required_preprocessors(&self) -> Vec<&'static str> {
        vec!["filetype"]
    }

    fn subscribed_filetypes(&self) -> Option<Vec<&'static str>> {
        Some(vec!["tiff", "jpg"])
    }
}
