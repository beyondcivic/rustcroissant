//! Error types for rustcroissant

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for rustcroissant operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for rustcroissant operations
#[derive(Error, Debug)]
pub enum Error {
    /// IO operation failed
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// CSV parsing failed
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// File not found
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    /// Invalid file format
    #[error("Invalid file format: {message}")]
    InvalidFormat { message: String },

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    /// Invalid output path
    #[error("Invalid output path: {path} - {reason}")]
    InvalidOutputPath { path: PathBuf, reason: String },

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    /// Invalid data type
    #[error("Invalid data type: {value} cannot be parsed as {data_type}")]
    InvalidDataType { value: String, data_type: String },

    /// Generic error
    #[error("Error: {0}")]
    Generic(String),
}

impl Error {
    /// Create a new generic error
    pub fn new(message: impl Into<String>) -> Self {
        Self::Generic(message.into())
    }

    /// Create a new file not found error
    pub fn file_not_found(path: impl Into<PathBuf>) -> Self {
        Self::FileNotFound { path: path.into() }
    }

    /// Create a new invalid format error
    pub fn invalid_format(message: impl Into<String>) -> Self {
        Self::InvalidFormat {
            message: message.into(),
        }
    }

    /// Create a new invalid output path error
    pub fn invalid_output_path(path: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        Self::InvalidOutputPath {
            path: path.into(),
            reason: reason.into(),
        }
    }

    /// Create a new missing field error
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Create a new invalid data type error
    pub fn invalid_data_type(value: impl Into<String>, data_type: impl Into<String>) -> Self {
        Self::InvalidDataType {
            value: value.into(),
            data_type: data_type.into(),
        }
    }
}

/// Convert anyhow::Error to our Error type
impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic(err.to_string())
    }
}
