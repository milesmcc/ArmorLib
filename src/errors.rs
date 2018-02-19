use std::fmt::{Display, Error, Formatter};
use std::error;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ArmorlibError {
    /// For use when an unknown processing error occurs.
    UnknownProcessingError(String),

    /// For use when a parsing error, such as that between a string and a number, occurs.
    /// The attached String is a human-meaningful description of the error.
    ParseError(String),

    /// For use when a preprocessor is missing. The attached `String` is the name of the
    /// preprocessor.
    MissingPreprocessor(String),

    /// For use when an expected key is missing from a preprocessor's metadata. The attached
    /// String is the key's path in the format `<preprocessor>/<key>`, following the convention
    /// defined by `ScanObject::get_metadata`.
    MissingMetadata(String),

    /// For use when an error occurs while reading a file.
    ReadFileError(String),
}

impl Display for ArmorlibError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let message: String = match self {
            // TODO: are these messages idiomatic? See https://github.com/milesmcc/ArmorLib/issues/4
            &ArmorlibError::UnknownProcessingError(ref msg) => {
                format!("an unknown processing error occured: {}", msg)
            }
            &ArmorlibError::ParseError(ref msg) => {
                format!("an error occured while parsing: {}", msg)
            }
            &ArmorlibError::MissingPreprocessor(ref msg) => {
                format!("unable to find the preprocessor `{}`", msg)
            }
            &ArmorlibError::MissingMetadata(ref msg) => {
                format!("unable to find the metadata at path `{}`", msg)
            }
            &ArmorlibError::ReadFileError(ref msg) => {
                format!("unable to read the file at path `{}`", msg)
            }
        };
        write!(f, "{}", message.as_str());
        Ok(()) // something seems off about this line
    }
}

impl error::Error for ArmorlibError {
    fn description(&self) -> &str {
        match self {
            &ArmorlibError::UnknownProcessingError(_) => "an unknown processing error occured",
            &ArmorlibError::ParseError(_) => "an error occured while parsing data",
            &ArmorlibError::MissingPreprocessor(_) => "unable to find the preprocessor",
            &ArmorlibError::MissingMetadata(_) => "unable to find the metadata",
            &ArmorlibError::ReadFileError(_) => "unable to read the file",
        }
    }
}
