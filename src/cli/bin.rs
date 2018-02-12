extern crate armorlib;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, Arg, SubCommand};

fn main() {
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(simplelog::LevelFilter::Info, simplelog::Config::default()).unwrap()
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
    unimplemented!();
}

fn run_modules(matches: &clap::ArgMatches) {
    unimplemented!();
}

fn run_preprocessors(matches: &clap::ArgMatches) {
    unimplemented!();
}
