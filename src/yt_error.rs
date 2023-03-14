use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum YTError {
    HttpError(String),
    StringParsingError(String),
    JSONParsingError(String),
    JSONValueConversionError(String),
}

impl YTError {
    pub fn to_yt_error(error: ureq::Error) -> YTError {
        match error {
            ureq::Error::Transport(e) => YTError::HttpError(e.to_string()),
            other => YTError::HttpError(other.to_string()),
        }
    }
}

impl Display for YTError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            other => f.write_str(&format!("Error with {}", other)),
        }
    }
}

impl From<ureq::Error> for YTError {
    fn from(error: ureq::Error) -> Self {
        YTError::to_yt_error(error)
    }
}

impl Error for YTError {}
