# Contributing a Scan Module

Scan modules are an important component of the ArmorLib system. They find security vulnerabilities, and generate `Finding` objects. To contribute a new scan module to ArmorLib, you'll need to follow these steps:

- Write your scan module
- Write comprehensive tests for your scan module
- Integrate your scan module in the ArmorLib architecture
- Submit a pull request

## Writing a Scan Module

Writing a scan module is simple. A scan module is any struct that the `ScanModule` trait has been implemented for. In this guide, we'll be creating an _unlucky file size_ scan module (using the `length` preprocessor created in the ['how to contribute a preprocessor' guide](PREPROCESSORS.md)). Our demo scan module will create a warning if the length of the file is a multiple of 13 or a multiple of 27.

### Saving the Scan Module

Save your scan module in the `src/scan_modules` directory under your scan module's name. For example, we're going to name this scan module `unlucky_length`--therefore, we'll want to call the scan module's file `src/scan_modules/unlucky_length.rs`.

### Necessary Imports

In order to write a scan module, we'll need to have several important items in scope.

First, we'll need to use the `ScanModule` trait:

```rust
use armorlib::ScanModule;
```

Then, because we'll be working quite a bit with `ScanObject`s, we'll use that too:

```rust
use armorlib::ScanObject;
```

Lastly, because the return type of `ScanModule::scan()` function is a `Result<Vec<Finding>, ArmorlibError>`, we'll want those too:

```rust
use armorlib::{Finding, ArmorlibError};
```

We're going to be converting from a `String` into a `usize` in our scan module, so we'll need to import `usize`:

```rust
use std::usize;
```

### Make a Struct

Now that we have everything in scope, we're ready to start writing our scan module. Because `ScanModule` is a _trait_, we need to create a _struct_ that we can implement the _trait_ for. A unit struct is sufficient here; in fact, it's suggested.

```rust
pub struct UnluckyLengthScanModule;
```

### Implement the Struct

Great--now we have a `struct` to work with. Let's implement it, and put in two `unimplemented!()` required functions for `ScanModule`s.

```rust
impl ScanModule for UnluckyLengthScanModule {
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
        unimplemented!();
    }

    fn info(&self) -> (&'static str, &'static str) {
        unimplemented!();
    }

    fn required_preprocessors(&self) -> Vec<&'static str> {
          unimplemented!();
      }
}
```

The `fn scan(&self, scan_object: &ScanObject)` is where the bulk of our ScanModule's functionality will be accessible. This is what ArmorLib calls to _run_ the scan module. It takes a reference to itself and a reference to a `ScanObject` as arguments, and returns a `Result<Vec<Finding>, ArmorlibError>`.

The `fn info(&self)` is where we give ArmorLib information about our scan module. We return a tuple where the first item is the _name_ of the scan module (in `snake_case`) and the second item is the _description_ of the scan module. Both should be human meaningful. Make sure that no scan module already exists that uses the same `name`--`name`s must be unique.

### Implementing `info()`

Let's start by implementing `fn info(&self)`:

```rust
fn info(&self) -> (&'static str, &'static str) {
    ("unlucky_length", "detects if the data's length in bytes is a multiple of 13 or 27")
}
```

We've implemented our `info` function! Now, let's move on to `fn scan(&self, scan_object: &ScanObject)`.

### Implementing `scan()`

Like we said in our scan module's description, we will alert the user if the data's length is a multiple of 13 or 27. Let's write a bit of documentation that explains as much.

```rust
//! This module defines the unlucky length scan module, which alerts the
//! user if the scan object's length in bytes is a multiple of `13` or `27`.

use armorlib::{ScanModule, ScanObject, Finding, ArmorlibError};
use std::usize;

pub struct UnluckyLengthScanModule;

impl ScanObject for UnluckyLengthScanModule {

    /// Process the given `&ScanObject` and warn if its data's length in
    /// bytes is a multiple of `13` or `27`.
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
        unimplemented!();
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("unlucky_length", "detects if the data's length in bytes is a multiple of 13 or 27")
    }

    fn required_preprocessors(&self) -> Vec<&'static str> {
          unimplemented!();
      }
}
```

Notice how we wrote both a module-level comment _and_ a comment on the `scan()` function.

Let's implement the `scan()` function:

```rust
fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
    let mut findings: Vec<Finding> = Vec::new();

    // get the length in bytes from the `length` preprocessor from the earlier tutorial
    let length_bytes = usize::from(scan_object.get_metadata("length/bytes")?).unwrap();

    if length_bytes % 13 == 0 {
      findings.push(Finding {
                title: String::from("length is multiple of 13"),
                description: format!("the length ({}) is a multiple of unlucky number 13", length_bytes),
                id: String::from("UNLUCKY_LENGTH_13"),
                severity: Severity::Warn(String::from("may indicate bad luck"))
            });
    }

    if length_bytes % 27 == 0 {
      findings.push(Finding {
                title: String::from("length is multiple of 27"),
                description: format!("the length ({}) is a multiple of unlucky number 27", length_bytes),
                id: String::from("UNLUCKY_LENGTH_27"),
                severity: Severity::Severe(String::from("likely indicates bad luck"))
            });
    }

    Ok(findings)
}
```

### Depending on Preprocessors

You may have noticed in the earlier example that we referred to a data from a preprocessor (the `length` preprocessor):

```rust
// get the length in bytes from the `length` preprocessor from the earlier tutorial
let length_bytes = usize::from(scan_object.get_metadata("length/bytes")?).unwrap();
```

We need to let ArmorLib know that we require the length preprocessor to run. (To save resources, ArmorLib won't run preprocessors if they are never used.)

To do this, we use the `required_preprocessors()` function and return a `Vec<&'static str'>`, where the `'static str'`s are the `name`s of the desired preprocessors:

```rust
fn required_preprocessors(&self) -> Vec<&'static str> {
    vec!["length"]
}
```

### Finishing the Module

We've now created a scan module. All together, it looks like this:

```rust
//! This module defines the unlucky length scan module, which alerts the
//! user if the scan object's length in bytes is a multiple of `13` or `27`.

use armorlib::{ScanModule, ScanObject, Finding, ArmorlibError};
use std::usize;

pub struct UnluckyLengthScanModule;

impl ScanObject for UnluckyLengthScanModule {

    /// Process the given `&ScanObject` and warn if its data's length in
    /// bytes is a multiple of `13` or `27`.
    fn scan(&self, scan_object: &ScanObject) -> Result<Vec<Finding>, ArmorlibError> {
        let mut findings: Vec<Finding> = Vec::new();

        // get the length in bytes from the `length` preprocessor from the earlier tutorial
        let length_bytes = usize::from(scan_object.get_metadata("length/bytes")?).unwrap();

        if length_bytes % 13 == 0 {
          findings.push(Finding {
                    title: String::from("length is multiple of 13"),
                    description: format!("the length ({}) is a multiple of unlucky number 13", length_bytes),
                    id: String::from("UNLUCKY_LENGTH_13"),
                    severity: Severity::Warn(String::from("may indicate bad luck"))
                });
        }

        if length_bytes % 27 == 0 {
          findings.push(Finding {
                    title: String::from("length is multiple of 27"),
                    description: format!("the length ({}) is a multiple of unlucky number 27", length_bytes),
                    id: String::from("UNLUCKY_LENGTH_27"),
                    severity: Severity::Severe(String::from("likely indicates bad luck"))
                });
        }

        Ok(findings)
    }

    fn info(&self) -> (&'static str, &'static str) {
        ("unlucky_length", "detects if the data's length in bytes is a multiple of 13 or 27")
    }

    fn required_preprocessors(&self) -> Vec<&'static str> {
        vec!["length"]
    }
}
```

It's simple!

### Writing Tests

We'll want to test our scan module to make sure everything works correctly. We'll do that by writing a simple `mod tests`—but that part will be left to you.

Once your tests pass, it's time to _integrate_ the scan module.

## Integrating the scan module

Even though you've saved your scan module, it's not yet going to be run by ArmorLib. To ensure that your scan module is accessible, you'll need to add it in `src/scan_modules/mod.rs`.

Add an import to your scan module in the second block of imports like so:

```rust
// List preprocessors here
[...]
pub mod unlucky_length;
```

Then, in `fn make_default_scan_modules()`, instantiate your scan module in the `vec`:

```rust
pub fn make_default_scan_modules() -> Vec<Box<ScanModule>> {
    vec![
        [...]
        Box::new(unlucky_length::UnluckyLengthScanModule {}),
        // ...and add additional scan modules here
    ]
}
```

And that's it—your scan module is ready to be used under the name `unlucky_length`!
