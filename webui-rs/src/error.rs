use std::error::Error;
use std::fmt;

use crate::{get_last_error_message, get_last_error_number};

/// The web ui error.
#[derive(Debug)]
pub struct WebUIError {
    /// The error code.
    code: usize,
    /// The error message.
    message: String,
}

impl WebUIError {
    pub(crate) fn get_last_error() -> Self {
        Self {
            code: get_last_error_number(),
            message: get_last_error_message(),
        }
    }

    pub(crate) fn from_bool(value: bool) -> Result<(), WebUIError> {
        if value {
            Ok(())
        } else {
            Err(WebUIError::get_last_error())
        }
    }
}

// Required 2
impl fmt::Display for WebUIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error (Code {}): {}", self.code, self.message)
    }
}

// Required 3
impl Error for WebUIError {}
