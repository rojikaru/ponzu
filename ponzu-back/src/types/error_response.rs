use std::fmt;
use serde::Serialize;

/// Serializable error object with error message and status code.
#[derive(Debug, Serialize)]
pub struct SerializableError {
    error: String,
    status: u16,
}

impl SerializableError {
    pub fn new(error: String, status: u16) -> Self {
        SerializableError { error, status }
    }
}

impl fmt::Display for SerializableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"error": "{}", "status": {}}}"#, self.error, self.status)
    }
}
