#[macro_use]
extern crate maplit;

extern crate aho_corasick;

pub mod coordinator;
pub mod binary_object;
pub mod scan_module;
pub mod scan_object;
pub mod scan_report;
pub mod scan_result;
pub mod finding;
pub mod errors;
pub mod preprocessor;

pub mod preprocessors;
pub mod scan_modules;

mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
