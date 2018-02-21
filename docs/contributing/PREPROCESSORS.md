# Contributing a Preprocessor

Preprocessors are an important component of the ArmorLib system. They prevent code duplication and make ArmorLib faster. Preprocessors create key-value data (structured as a `HashMap<String, String>`) that is accessible to the scan modules. To contribute a preprocessor to ArmorLib, you'll need to follow these steps:

- Write your preprocessor
- Write comprehensive tests for your preprocessor
- Integrate your preprocessor in the ArmorLib architecture
- Submit a pull request

## Writing a Preprocessor

Writing a preprocessor is simple. A preprocessor is any struct that the `Preprocessor` trait has been implemented for. In this guide, we'll be creating a _length_ preprocessor. It will calculate the length of the data, and then make this data available to scan modules.

### Saving the Preprocessor

Save your preprocessor in the `src/preprocessors` directory under your preprocessor's name. For example, we're going to name this preprocessor `length`--therefore, we'll want to call the preprocessor's file `src/preprocessors/length.rs`.

### Necessary Imports

In order to write a preprocessor, we'll need to have several important items in scope.

First, we'll need to use the Preprocessor trait:

```rust
use armorlib::Preprocessor;
```

Then, because we'll be working quite a bit with `BinaryObject`s, we'll use that too:

```rust
use armorlib::BinaryObject;
```

Lastly, because the return type of `preprocessor::process()` function is a `HashMap<String, String>`, we'll want that too:

```rust
use std::collections::HashMap;
```

### Make a Struct

Now that we have everything in scope, we're ready to start writing our preprocessor. Because `Preprocessor` is a _trait_, we need to create a _struct_ that we can implement the _trait_ for. A unit struct is sufficient here; in fact, it's suggested.

```rust
pub struct LengthPreprocessor;
```

### Implement the Struct

Great--now we have a `struct` to work with. Let's implement it, and put in two `unimplemented!()` required functions for `Preprocessor`s.

```rust
impl Preprocessor for LengthPreprocessor {
    fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String> {
        unimplemented!();
    }

    fn info(&self) -> (&'static str, &'static str) {
        unimplemented!();
    }
}
```

The `fn process(&self, binary_object: &BinaryObject)` is where the bulk of our functionality will be accessible. This is what ArmorLib calls to _run_ the preprocessor. It takes a reference to itself and a reference to a `BinaryObject` as arguments, and returns a `HashMap<String, String>`.

The `fn info(&self)` is where we give ArmorLib information about our Preprocessor. We return a tuple where the first item is the _name_ of the preprocessor (in `snake_case`) and the second item is the _description_ of the preprocessor. Both should be human meaningful. Make sure that no preprocessor already exists that uses the same `name`--`name`s must be unique.

### Implementing `info()`

Let's start by implementing `fn info(&self)`:

```rust
fn info(&self) -> (&'static str, &'static str) {
    ("length", "calculates the length of the data in bits and bytes")
}
```

We've implemented our `info` function! Now, let's move on to `fn process(&self, binary_object: &BinaryObject)`.

### Implementing `process()`

Like we said in our preprocessor's description, we calculate the length of the data in both bits and bytes. Let's write a bit of documentation that explains as much.

```rust
//! This module defines the length preprocessor, which calculates the length of
//! the given binary object in both bits and bytes.

use armorlib::{Preprocessor, BinaryObject};
use std::collections::HashMap;

pub struct LengthPreprocessor;

impl Preprocessor for LengthPreprocessor {

    /// Process the given `&BinaryObject` and calculate its length in both
    /// bits and bytes. The number of `bits` are included under the field
    /// the field `bits`, and the number of `bytes` are included under the
    /// field `bytes`.
    fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String> {
        unimplemented!();
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("length", "calculates the length of the data in bits and bytes")
    }
}
```

Notice how we wrote both a module-level comment _and_ a comment on the `process()` function. Also note how we explicitly explained the fields we were creating in the `process()` function.

Finally, let's implement the `process()` function.

```rust
fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String> {
    let length_bytes = binary_object.data.len();
    let length_bits = 8 * length_bytes;

    hashmap!{
      String::from("bits") => String::from(length_bits),
      String::from("bytes") => String::from(length_bytes)
    }
}
```

We've now created a preprocessor. All together, it looks like this:

```rust
//! This module defines the length preprocessor, which calculates the length of
//! the given binary object in both bits and bytes.

use armorlib::{Preprocessor, BinaryObject};
use std::collections::HashMap;

pub struct LengthPreprocessor;

impl Preprocessor for LengthPreprocessor {

    /// Process the given `&BinaryObject` and calculate its length in both
    /// bits and bytes. The number of `bits` are included under the field
    /// the field `bits`, and the number of `bytes` are included under the
    /// field `bytes`.
    fn process(&self, binary_object: &BinaryObject) -> HashMap<String, String> {
        let length_bytes = binary_object.data.len();
        let length_bits = 8 * length_bytes;

        hashmap!{
          String::from("bits") => String::from(length_bits),
          String::from("bytes") => String::from(length_bytes)
        }
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("length", "calculates the length of the data in bits and bytes")
    }
}
```

That was pretty idiomatic, no?

### Writing Tests

We'll want to test our preprocessor to make sure everything works correctly. We'll do that by writing a simple `mod tests`.

```rust
#[cfg(test)]
mod tests {
    use binary_object::BinaryObject;
    use super::*;

    #[test]
    fn test_length_preprocessor() {
        let results = process(BinaryObject::from(vec![0,0,0])); // 3 bytes, 24 bits
        assert_eq!(results.get("bits").unwrap(), String::from("24"));
        assert_eq!(results.get("bytes").unwrap(), String::from("3"));
    }
}
```

Once your tests pass, it's time to _integrate_ the preprocessor.

## Integrating the Preprocessor

Even though you've saved your preprocessor, it's not yet going to be run by ArmorLib. To ensure that your preprocessor is accessible, you'll need to add it in `src/preprocessors/mod.rs`.

Add an import to your preprocessor in the second block of imports like so:

```rust
// List preprocessors here
[...]
pub mod length;
```

Then, in `fn make_default_scan_modules()`, instantiate your preprocessor in the `vec`:

```rust
pub fn make_default_scan_modules() -> Vec<Box<ScanModule>> {
    vec![
        [...]
        Box::new(length::LengthPreprocessor {}),
        // ...and add additional preprocessors here
    ]
}
```

And that's it—your preprocessor is ready to be used under the name `length`.
