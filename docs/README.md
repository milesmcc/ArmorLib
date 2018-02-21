# ArmorLib Documentation
ArmorLib has two parts: a command line tool for general usage, and a Rust library for those who wish to integrate ArmorLib's functionality into their own code or extend ArmorLib themselves. These two fundamentally different use cases are addressed separately in the ArmorLib documentation. Contributing to ArmorLib is also addressed separately.

## Command Line Interface
To learn how to use ArmorLib's powerful command line interface (`CLI`), start by visiting the CLI ['Getting Started' page](cli/GETTING_STARTED.md).

Alternatively, you can access documentation right from the shell! Install ArmorLib with Cargo by running `cargo install armorlib` and then run `armorlib --help`:

```
$ armorlib --help
ArmorLib CLI 0.1.0
R. Miles McCain <libre@rmrm.io>
Easily scan files for threats to security and privacy.

USAGE:
    armorlib [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help             Prints this message or the help of the given subcommand(s)
    modules          get information about the available scan modules
    preprocessors    get information about the available preprocessors
    scan             scans the file at the given path
```

## Library
To learn how to use ArmorLib in your own Rust library, two major resources are offered: `rustdoc` and the library's ['Getting Started' guide](library/GETTING_STARTED.md).

If you would like to learn more about the high-level structure of ArmorLib, refer to [STRUCTURE.md](STRUCTURE.md).

### Rustdoc

The `rustdoc` is an interactive documentation website that, if you're familiar with Rust, you'll be used to. It documents every single public function, struct, enum, and field of ArmorLib and provides dozens of examples to work with. The `rustdoc` is perfect for exploring the ArmorLib source and learning how to interact with its internals. If you're looking for a step-by-step guide to use ArmorLib in your own project, consider reading the guide below.

View the `rustdoc` [here](doc/armorlib/index.html).

### Getting Started Guide
If you're looking to quickly get started with using ArmorLib in your own Rust project, read the ['Getting Started' guide](lib/GETTING_STARTED.md). It will guide you through the process of installing ArmorLib and scanning files and custom hex data.

---

To learn about the high level architecture of ArmorLib, see [STRUCTURE.md](STRUCTURE.md). For the development ideology, see [IDEOLOGY.md](IDEOLOGY.md).
