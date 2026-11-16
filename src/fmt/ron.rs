//! RON serialization support.
use serde::{Serialize, de::DeserializeOwned};

use crate::{ConfigError, ConfigurationFormat, Result};
/// RON formatter.
#[derive(Debug, Default, Clone, Copy)]
pub struct RonFormat;
impl ConfigurationFormat for RonFormat {
  fn decode<T: DeserializeOwned>(content: &str) -> Result<T> {
    ron::from_str(content).map_err(|e| ConfigError::new("RON", e.to_string()))
  }
  fn encode<T: Serialize>(value: &T) -> Result<String> {
    ron::to_string(value).map_err(|e| ConfigError::new("RON", e.to_string()))
  }
}
