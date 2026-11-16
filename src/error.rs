//! Shared errors returned by format parsers and serializers.

use std::{error::Error as StdError, fmt};

/// Error produced while parsing or serializing configuration data.
#[derive(Debug)]
pub struct ConfigError {
  format: &'static str,
  message: String,
}

impl ConfigError {
  /// Creates an error with a format name and message.
  pub fn new(format: &'static str, message: impl Into<String>) -> Self {
    Self { format, message: message.into() }
  }

  /// Returns the format that produced this error.
  #[must_use]
  pub const fn format(&self) -> &'static str { self.format }
}

impl fmt::Display for ConfigError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} configuration error: {}", self.format, self.message)
  }
}

impl StdError for ConfigError {}

/// Result type used by this crate.
pub type Result<T> = std::result::Result<T, ConfigError>;
