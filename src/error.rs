// Use the std traits 'Display' and 'Formatter' from the standard library fmt module
use std::fmt::{Display, Formatter};

// Create an enum 'Error' and derive the Debug trait
#[derive(Debug)]
pub enum Error {
    // Create variants that take a reqwest::Error or a serde_json::Error
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    // Add a variant with no payload
    NoChoicesError,
}

// Implement Display for Error
impl Display for Error {
    // Implement a custom formatting for each error variant
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Match self and format the payload of each error variant
        match self {
            Error::RequestError(err) => 
                write!(f, "{}", err),
            Error::JsonError(err) => 
                write!(f, "{}", err),
            // For the NoChoicesError variant return a string
            Error::NoChoicesError => write!(f, "no choices received"),
        }
    }
}

// Implement From trait for reqwest::Error and serde_json::Error
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::RequestError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

// Implement std::error::Error for Error
impl std::error::Error for Error {}