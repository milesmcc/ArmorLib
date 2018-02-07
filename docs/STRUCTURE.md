# High-Level Structure
ArmorLib takes the following high-level structure to maximize modularity and extendability. This document details the implementation-independent structure of ArmorLib to bring prospective developers up to speed on the software architecture.

At the most general level, ArmorLib takes in a binary object—a text file, a PDF, a ZIP file, or anything else—and returns to the user a security and privacy report about that file (or the files that it contains, in the case of an archive file). ArmorLib performs its file scanning in _scan modules_, small self-contained programs that take in a binary object and return a security report. The scan modules are managed by the _coordinator_, which ensures that binary objects provided by the user are passed through the right scan modules. (For example, the coordinator will ensure that a text file isn't given to a scan module meant for `.docx` files.) Binary objects are prepared for processing by _preprocessors_, which run before the scan modules and do _not_ return reports but instead append metadata to the binary object. For example, a core preprocessor would determine the binary object's ISO filetype classification, while another may determine whether or not a file is encrypted.

## Coordinator
The _coordinator_ manages the lifecycle of an ArmorLib scan. When the coordinator is given a blob to process (either by an API call or via a command-line invocation), the coordinator first runs all preprocessors. Then, the coordinator passes the prepared binary object (with metadata from the preprocessors) through each scan module.

## Preprocessors
_Preprocessors_ take in a binary object and return metadata to the coordinator. Preprocessors may not rely on any other preprocessors. One preprocessor may determine the file's type (necessary for determining which scan modules will run), while another might determine whether the file is encrypted. Preprocessors generally perform functions that create data that is useful to a wide number of scan modules.

## Scan Modules
The core modular unit of ArmorLib is the _scan module_. A scan module takes a binary object—typically, the file the user is scanning—as an input and returns a _scan report_ as an output. Scan modules perform the actual _work_ in ArmorLib. For example, a certain scan module searches for the presence of zero-width characters in text files, while another may search for hidden virus payloads in _all_ files. Scan modules can range from very general (those that are run on _every_ file scanned) to very specific (those that only run on PDFs, for example). Scan modules may be recursive—that is, they may call the on coordinator to return a report on a separate binary file (for example, the _.ZIP contents_ scan module may recursively run scans on all of the .zip file's contents).

Scan modules may be called as part of a full scan as managed by the _coordinator_, or independently by another program by direct import. While the latter process must also be performed through the coordinator (preprocessors still must be run, after all). Therefore, scan modules may not depend on any other scan modules to function (though they may depend on the preprocessors specified in their configuration; see below).

### Registration
All scan modules must be recognized by the coordinator in order to be run when ArmorLib scans a file. In order for a scan module to be recognized, it must be referenced in the ArmorLib configuration. In registration, the scan module tells the coordinator its location (that is to say, how to _run_ it), its _subscription_ (the type of files it handles, as an ISO code), and the preprocessors it depends on.

## Scan Reports
Each scan module returns a _scan report_ to the coordinator. A scan report is an array of findings, each of which has the following fields: `status` (an enumerator for `OK`, `WARN`, `DANGER`, `SEVERE` that corresponds to the most severe `status` of the any of the findings); and `findings`, an array of findings, each of which has a human-readable description, a `status` level (an enumerator identical to that of the parent object), a `title`, a `type` (human meaningful and constant among all identical findings, for example `ZERO_WIDTH_CHARACTER_DETECTED`), and an unstructured JSON-like object `data` (the `.ZIP contents` scan report might attach the scan reports of the archive file's contents in the `data` field, for example).

## Command Line Interface
The _command line interface_ allows for an end-user to interact directly with the coordinator without using another Rust program as a hook. It acts as a proxy by which the user can run files on the local machine or on the network through the coordinator, and parses the scan reports into a human-readable format.
