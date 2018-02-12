use std::collections::HashMap;

use binary_object::BinaryObject;

pub trait Preprocessor {
    fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String>;
    /// Returns a tuple of the name and description of the preprocessor.
    fn info(&self) -> (&'static str, &'static str);
    fn name(&self) -> &'static str {
        return self.info().0;
    }
    fn description(&self) -> &'static str {
        return self.info().1;
    }
}
