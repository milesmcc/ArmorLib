extern crate serde;

pub mod coordinator;
pub mod binary_object;
pub mod scan_module;
pub mod scan_object;
pub mod scan_report;
pub mod scan_result;
pub mod finding;
pub mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
