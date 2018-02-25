//! This file allows for more fine-grained access to the ArmorLib system. While in most cases
//! it is suitable to simply call `.process()` on a `File`, `Vec<u8>`, or `BinaryObject`, there
//! exist some cases when more control is needed. For example, if you'd like to run only a select
//! number of scan modules (instead of all), you can use the detailed `process()` function defined
//! in this module to do so.

use scan_result::ScanResult;
use binary_object::BinaryObject;
use errors::ArmorlibError;
use preprocessor::Preprocessor;
use scan_module::ScanModule;
use scan_object::ScanObject;
use preprocessors;
use scan_modules;

/// Process the given `BinaryObject` through the ArmorLib system and return a `ScanResult`.
/// All other `process()` functions are simply wrappers for this function.
///
/// # Arguments
///
/// * `scan_modules_to_run`: a vec of the `ScanModule`s to run while processing.
/// * `extra_preprocessors`: a vec of the extra preprocessors (in addition to those that are
/// included in the core library) that are necessary to run the given `scan_mdoules_to_run`. Like
/// normal core library preprocessors, a preprocessor will only run if it is required by some scan
/// module.
/// * `binary_object`: the `BinaryObject` that will be scanned by ArmorLib.
/// * `filetype`: an `Option<String>` of the filetype of the data, without the preceding `.`
/// (e.g. `file.pdf` -> `pdf`). Because this information may not be known, there is a possibility
/// of absence that is represented by the wrapping `Option`.
///
/// # Examples
///
/// ```rust
/// use armorlib::{scan_modules, binary_object, util};
/// use armorlib::coordinator;
/// let scan_result =
///    coordinator::process(
///        scan_modules::make_default_scan_modules(),
///        Vec::new(),
///        binary_object::BinaryObject::from(
///            util::hex_to_vec(
///                "48 8B CD E8 60 FF FF FF 48 FF C3 32 44 1E FF 48 FF CF 88 43 FF 48 8B CD E8 60"
///            ).unwrap()),
///            None
///        );
/// ```

pub fn process(
    scan_modules_to_run: Vec<Box<ScanModule>>,
    mut extra_preprocessors: Vec<Box<Preprocessor>>,
    binary_object: BinaryObject,
    filetype: Option<String>,
) -> Result<ScanResult, ArmorlibError> {
    let mut required_preprocessors: Vec<String> = Vec::new();

    // figure out which preprocessors are required
    for sm in &scan_modules_to_run {
        for required_preprocessor in sm.required_preprocessors() {
            let rp_as_string = String::from(required_preprocessor);
            if !required_preprocessors.contains(&rp_as_string) {
                required_preprocessors.push(rp_as_string);
            }
        }
    }

    // put together a vec of the preprocessors to be run
    let mut preprocessors_to_be_run: Vec<Box<Preprocessor>> = Vec::new();
    let mut available_preprocessors: Vec<Box<Preprocessor>> = Vec::new();

    available_preprocessors.append(&mut extra_preprocessors);
    available_preprocessors.append(&mut preprocessors::make_default_preprocessors());

    for preprocessor in available_preprocessors {
        if required_preprocessors.contains(&String::from(preprocessor.name()))
            || preprocessor.name() == &String::from("filetype")
        // filetype must always be run
        {
            preprocessors_to_be_run.push(preprocessor);
        }
    }

    // run the preprocessors
    let object_metadata = preprocessors::process(preprocessors_to_be_run, &binary_object);

    let mut filetypes: Vec<String> = Vec::new();
    {
        // establish _real_ filetypes
        let file_keys = match (&object_metadata).get("filetype") {
            Some(hashmap) => hashmap.keys(),
            None => {
                return Err(ArmorlibError::MissingPreprocessor(String::from(
                    "cannot find critical `filetype` preprocessor",
                )))
            }
        };

        for ft in file_keys {
            filetypes.push(ft.clone());
        }
    }

    // create scan object
    let scan_object: ScanObject = ScanObject {
        binary_object: binary_object,
        filetype: filetype,
        detected_filetypes: filetypes,
        metadata: object_metadata,
    };

    // run scan modules
    let scan_result = scan_modules::process(scan_modules_to_run, &scan_object);

    Ok(scan_result)
}

#[cfg(test)]
mod tests {
    use scan_modules;
    use binary_object;
    use util;
    use coordinator::process;

    #[test]
    fn test_full_cycle() {
        let _scan_result =
            process(
                scan_modules::make_default_scan_modules(),
                Vec::new(),
                binary_object::BinaryObject::from(
                    util::hex_to_vec(
                        "48 8B CD E8 60 FF FF FF 48 FF C3 32 44 1E FF 48 FF CF 88 43 FF 48 8B CD E8 60 FF FF FF 48 FF C3 32 44 1E FF 48 FF CF 88 43 FF 48 8B CD E8 60 FF FF FF 48 FF C3 32 44 1E FF 48 FF CF 88 43 FF"
                    ).unwrap()),
                    None
                ).unwrap();
    }
}
