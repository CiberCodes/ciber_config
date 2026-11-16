//! CBR serialization support.
use serde::{Serialize, de::DeserializeOwned};

use crate::{ConfigError, ConfigurationFormat, Result};

/// CBR formatter.
#[derive(Debug, Default, Clone, Copy)]
pub struct CbrFormat;

impl ConfigurationFormat for CbrFormat {
  fn decode<T: DeserializeOwned>(content: &str) -> Result<T> {
    ciber_serde::from_str(content)
      .map_err(|e| ConfigError::new("CBR", e.to_string()))
  }
  fn encode<T: Serialize>(value: &T) -> Result<String> {
    ciber_serde::to_string(value)
      .map_err(|e| ConfigError::new("CBR", e.to_string()))
  }
}
