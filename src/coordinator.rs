use scan_result::ScanResult;
use binary_object::BinaryObject;
use errors::ProcessingError;
use preprocessor::Preprocessor;
use scan_module::ScanModule;
use scan_object::ScanObject;
use preprocessors;
use scan_modules;

/// Process the given `BinaryObject` and return a `ScanResult`
pub fn process(
    scan_modules_to_run: Vec<Box<ScanModule>>,
    mut extra_preprocessors: Vec<Box<Preprocessor>>,
    binary_object: BinaryObject,
    filetype: Option<String>,
) -> Result<ScanResult, ProcessingError> {
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
        if required_preprocessors.contains(&String::from(preprocessor.name())) {
            preprocessors_to_be_run.push(preprocessor);
        }
    }

    // run the preprocessors
    let object_metadata = preprocessors::process(preprocessors_to_be_run, &binary_object);

    // create scan object
    let scan_object: ScanObject = ScanObject {
        binary_object: binary_object,
        filetype: filetype,
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
        let scan_result =
            process(
                scan_modules::make_default_scan_modules(),
                Vec::new(),
                binary_object::BinaryObject::from(
                    util::hex_to_vec(
                        "48 8B CD E8 60 FF FF FF 48 FF C3 32 44 1E FF 48 FF CF 88 43 FF 48 8B CD E8 60 FF FF FF 48 FF C3 32 44 1E FF 48 FF CF 88 43 FF 48 8B CD E8 60 FF FF FF 48 FF C3 32 44 1E FF 48 FF CF 88 43 FF"
                    ).unwrap()),
                    None
                );
        println!("test");
        println!("{:?}", scan_result);
    }
}
