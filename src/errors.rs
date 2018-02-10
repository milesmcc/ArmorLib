#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ProcessingError {
    UnknownProcessingError(String),
    ParseError(String),
}
