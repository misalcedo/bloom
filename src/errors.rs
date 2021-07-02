use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// An error that occurred while interacting with a Bloom Fitler.
#[derive(Debug)]
pub struct BloomFilterError {
    pub kind: ErrorKind,
}

impl BloomFilterError {
    /// Creates a new error for the given error kind.
    pub fn new(kind: ErrorKind) -> Self {
        BloomFilterError { kind }
    }
}

impl Display for BloomFilterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "An error occurred: {}", self.kind)
    }
}

impl Error for BloomFilterError {}

/// The kind of error that occurred in interacting with a Bloom Filter.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ErrorKind {
    /// Returned when the operation is not supported on the bloom filter implementation.
    NotSupported,
    /// Returned when there is an issue with a call to the async runtime.
    AsyncRuntime,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ErrorKind> for BloomFilterError {
    fn from(kind: ErrorKind) -> Self {
        BloomFilterError::new(kind)
    }
}
