use std::fmt::{Display, Error, Formatter};
use std::error;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ProcessingError {
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
}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let message: String = match self {
            // TODO: are these messages idiomatic? See https://github.com/milesmcc/ArmorLib/issues/4
            &ProcessingError::UnknownProcessingError(ref msg) => {
                format!("an unknown processing error occured: {}", msg)
            }
            &ProcessingError::ParseError(ref msg) => {
                format!("an error occured while parsing: {}", msg)
            }
            &ProcessingError::MissingPreprocessor(ref msg) => {
                format!("unable to find the preprocessor `{}`", msg)
            }
            &ProcessingError::MissingMetadata(ref msg) => {
                format!("unable to find the metadata at path `{}`", msg)
            }
        };
        write!(f, "{}", message.as_str())?;
        Ok(()) // something seems off about this line
    }
}

impl error::Error for ProcessingError {
    fn description(&self) -> &str {
        match self {
            &ProcessingError::UnknownProcessingError(_) => "an unknown processing error occured",
            &ProcessingError::ParseError(_) => "an error occured while parsing data",
            &ProcessingError::MissingPreprocessor(_) => "unable to find the preprocessor",
            &ProcessingError::MissingMetadata(_) => "unable to find the metadata",
        }
    }
}
