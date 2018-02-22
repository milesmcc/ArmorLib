# Getting Started with ArmorLib as a Library

ArmorLib is designed to be simple from the outside, but powerful and extendable on the inside. This guide will teach you how to use ArmorLib in your own Rust programs.

> **Looking to contribute to ArmorLib?** Read this guide, and then check out the [contributing guidelines and resources](/CONTRIBUTING.md).

## Quick Start

It's very easy to get started with ArmorLib. All you have to do is import it into scope and then use the `process()` function on a `std::fs::File` or a `Vec<u8>`:

```rust
extern crate armorlib;
use armorlib::Process;
use std::fs::File;

let results = File::open("/path/to/your/file").unwrap().process();
```

That `results` object holds everything that ArmorLib found about your file. Here's an example on how you'd go about interacting with the data (taken from the command line interface):

```rust
let result = match results {
      Ok(result) => result,
      Err(error) => {
          error!("an error occurred while processing: {}", error);
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
```

If you'd like to dive deeper into ArmorLib's API, read the `rustdoc` at [armorlib.org](https://armorlib.org/docs/doc/armorlib/).
