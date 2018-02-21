//! This module defines the `Preprocessor` trait, which allows for generalized data to be created
//! about `BinaryObject`s prior to being run through the `ScanModule`s to avoid duplicate
//! processing. For more information on how to contribute your own preprocessor, see
//! `docs/contributing/PREPROCESSORS.md`.

use std::collections::HashMap;
use binary_object::BinaryObject;

/// A trait that defines the necessary functions of a `Preprocessor`. A `Preprocessor` is a
/// modular component that has no dependencies on other preprocessors. It creates a HashMap
/// of `String`s mapped to other `String`s that can then be accessed by the scan modules.
/// For more information on how to contribute your own preprocessor, see
/// `docs/contributing/PREPROCESSORS.md`.
pub trait Preprocessor {
    /// Process the given `BinaryObject` and return a HashMap of `String`s.
    fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String>;

    /// Returns a tuple of the name and description of the preprocessor. It is important that
    /// the first item of the tuple, the name, does not change once a preprocessor has been
    /// added to the master ArmorLib codebase, as the name is what scan modules rely on to
    /// ensure that the preprocessor's data is in scope.
    fn info(&self) -> (&'static str, &'static str);

    /// Returns a `&'static str` of the name of the preprocessor. This is the name that the
    /// scan modules will rely on to ensure that the preprocessor's data is available, so it
    /// is important that this information does not change.
    fn name(&self) -> &'static str {
        return self.info().0;
    }

    /// Returns a `&'static str` of the description of the preprocessor.
    fn description(&self) -> &'static str {
        return self.info().1;
    }
}
