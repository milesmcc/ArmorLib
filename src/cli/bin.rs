extern crate armorlib;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, Arg, SubCommand};

fn main() {
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(simplelog::LevelFilter::Info, simplelog::Config::default())
            .unwrap(),
    ]).unwrap();

    let matches = App::new("ArmorLib CLI")
        .version(crate_version!())
        .about("Easily scan files for threats to security and privacy.")
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("scan")
                .about("scans the file at the given path")
                .arg(
                    Arg::with_name("path")
                        .help("Sets the path of the file to be scanned")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("output_location")
                        .help("Sets the path of the output file")
                        .required(false)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("modules")
                .about("get information about the available scan modules"),
        )
        .subcommand(
            SubCommand::with_name("preprocessors")
                .about("get information about the available preprocessors"),
        )
        .get_matches();
    run(matches);
}

fn run(matches: clap::ArgMatches) {
    match matches.subcommand() {
        ("scan", Some(m)) => run_scan(m),
        ("modules", Some(m)) => run_modules(m),
        ("preprocessors", Some(m)) => run_preprocessors(m),
        _ => error!("no command specified; try running with `--help`."),
    }
}

fn run_scan(matches: &clap::ArgMatches) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::ffi::OsStr;
    use std::path::Path;
    use armorlib::finding::Severity;

    let path: String = String::from(matches.value_of("path").unwrap()); // safe to unwrap, CLAP makes sure of it

    info!("loading file {}", path);

    let mut f = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            error!("unable to open {}: {}", path, error);
            return;
        }
    };
    let mut contents: Vec<u8> = Vec::new();
    match f.read_to_end(&mut contents) {
        Ok(size) => info!("successfully read {} bytes", size),
        Err(error) => {
            error!("unable to read {}: {}", path, error);
            return;
        }
    }

    let parsed_path: &Path = Path::new(&path);
    let file_extension: Option<String> = match parsed_path.extension() {
        Some(osstring) => match osstring.to_str() {
            Some(string) => Some(String::from(string)),
            None => None,
        },
        None => None,
    };

    let scan_modules = armorlib::scan_modules::make_default_scan_modules();

    info!(
        "processing with {} scan modules and {} available preprocessors...",
        scan_modules.len(),
        armorlib::preprocessors::make_default_preprocessors().len()
    );

    let results = armorlib::coordinator::process(
        scan_modules,
        Vec::new(),
        armorlib::binary_object::BinaryObject {
            data: contents,
            file_path: match parsed_path.canonicalize() {
                Ok(buf) => match buf.to_str() {
                    Some(string) => Some(String::from(string)),
                    None => {
                        error!("unnable to canonicalize path {}", path);
                        return;
                    }
                },
                Err(error) => {
                    error!("unable to canonicalize path {}: {}", path, error);
                    return;
                }
            },
        },
        file_extension,
    );

    let result = match results {
        Ok(result) => result,
        Err(error) => {
            error!("an error occured while processing: {}", error);
            return;
        }
    };

    info!("RESULTS ------------------------------------");

    for report in result.reports {
        info!("→ {} ({})", report.module_info.0, report.module_info.1);
        match report.error {
            Some(error) => warn!("    encountered an error: {}", error),
            None => info!("    scanned successfully"),
        }

        match report.findings {
            Some(findings) => {
                info!("    had {} findings:", findings.len());
                for finding in findings {
                    info!("        - {} ({})", finding.title, finding.id);
                    info!("          {}", finding.description);
                    match finding.severity {
                        Severity::Ok(description) => info!("          OK: {}", description),
                        Severity::Warn(description) => warn!("          WARNING: {}", description),
                        Severity::Danger(description) => warn!("          DANGER: {}", description),
                        Severity::Severe(description) => warn!("          SEVERE: {}", description),
                    }
                }
            }
            None => info!("    (had no findings)"),
        }
    }
}

fn run_modules(matches: &clap::ArgMatches) {
    let modules = armorlib::scan_modules::make_default_scan_modules();
    info!(
        "there are currently {} scan modules available:",
        modules.len()
    );
    for module in modules {
        info!("    → {}: {}", module.name(), module.description());
    }
}

fn run_preprocessors(matches: &clap::ArgMatches) {
    let preprocessors = armorlib::preprocessors::make_default_preprocessors();
    info!(
        "there are currently {} preprocessors available:",
        preprocessors.len()
    );
    for preprocessor in preprocessors {
        info!(
            "    → {}: {}",
            preprocessor.name(),
            preprocessor.description()
        );
    }
}
