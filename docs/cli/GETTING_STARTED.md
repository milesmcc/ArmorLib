# Using ArmorLib on the command line
You've installed ArmorLib as a command-line utility. Great! Now what? This page will explain how you can use ArmorLib as a command-line tool.

Before you can use ArmorLib on the command line, you'll need to make sure that your shell can find the ArmorLib executable. Usually, `cargo` will do this for you. If it doesn't, see [this](https://doc.rust-lang.org/book/second-edition/ch14-04-installing-binaries.html) Rust guide.

### Documentation on the command line
Running the ArmorLib executable with no arguments should yield the following response:

```
$ armorlib
01:59:35 [ERROR] no command specified; try running with `--help`.
```

(Note that the time—given at the beginning of the line—will be different depending on _when_ you run the command.)

Let's do as ArmorLib says, though, and run with `--help`. This will give us information about how to use ArmorLib in a standard format. (It should feel familiar.) ArmorLib uses the `clap` library for command line argument parsing, so typos, for example, are given automatic suggestions. Running with `--help` gives the following:

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

### Scanning a file
The core command-line functionality is scanning a file. To learn more about the `scan` command, run `armorlib help scan`:

```
$ armorlib help scan
armorlib-scan
scans the file at the given path

USAGE:
    armorlib scan <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>    Sets the path of the file to be scanned
```

To scan a file, then, you would run `armorlib scan <path>` and receive something like the following:

```
$ armorlib scan ~/file.dat
03:49:09 [INFO] loading file  ~/file.dat
03:49:09 [INFO] successfully read 234440334 bytes
03:49:09 [INFO] processing with 2 scan modules and 3 available preprocessors...
03:49:09 [INFO] RESULTS ------------------------------------
03:49:09 [INFO] → strings (searches for suspicious byte patterns)
03:49:09 [INFO]     scanned successfully
03:49:09 [INFO]     (had no findings)
03:49:09 [INFO] → unicode_watermark (searches for attempts to watermark text using unusual Unicode)
03:49:09 [INFO]     scanned successfully
03:49:09 [INFO]     (had no findings)
```

(You may receive slightly different results depending on the scan modules available on your system and the type of file you scan.)

### Viewing the available preprocessors
To view the available preprocessors, run `armorlib preprocessors`:

```
$ armorlib preprocessors
03:52:34 [INFO] there are currently 3 preprocessors available:
03:52:34 [INFO]     → filetype: determines the filetype using magic numbers
03:52:34 [INFO]     → text: intelligently extracts strings
03:52:34 [INFO]     → hex: creates a hexadecimal byte representation
```

(You may receive slightly different results depending on the preprocessors available on your system.)

### Viewing the available scan modules
To view the available scan modules, run `armorlib modules`:

```
$ armorlib modules
03:53:35 [INFO] there are currently 2 scan modules available:
03:53:35 [INFO]     → strings: searches for suspicious byte patterns
03:53:35 [INFO]     → unicode_watermark: searches for attempts to watermark text using unusual Unicode
```

(You may receive slightly different results depending on the scan modules available on your system.)
