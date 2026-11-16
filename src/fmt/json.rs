//! JSON serialization support.
use serde::{Serialize, de::DeserializeOwned};

use crate::{ConfigError, ConfigurationFormat, Result};
/// JSON formatter.
#[derive(Debug, Default, Clone, Copy)]
pub struct JsonFormat;
impl ConfigurationFormat for JsonFormat {
  fn decode<T: DeserializeOwned>(content: &str) -> Result<T> {
    serde_json::from_str(content)
      .map_err(|e| ConfigError::new("JSON", e.to_string()))
  }
  fn encode<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value)
      .map_err(|e| ConfigError::new("JSON", e.to_string()))
  }
}
