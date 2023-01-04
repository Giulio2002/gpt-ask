use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequestError(err) => 
                write!(f, "{}", err),
            Error::JsonError(err) => 
                write!(f, "{}", err),
        }
    }
}

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

impl std::error::Error for Error {}
