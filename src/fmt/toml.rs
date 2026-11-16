//! TOML serialization support.
use serde::{Serialize, de::DeserializeOwned};

use crate::{ConfigError, ConfigurationFormat, Result};
/// TOML formatter.
#[derive(Debug, Default, Clone, Copy)]
pub struct TomlFormat;
impl ConfigurationFormat for TomlFormat {
  fn decode<T: DeserializeOwned>(content: &str) -> Result<T> {
    toml::from_str(content).map_err(|e| ConfigError::new("TOML", e.to_string()))
  }
  fn encode<T: Serialize>(value: &T) -> Result<String> {
    toml::to_string(value).map_err(|e| ConfigError::new("TOML", e.to_string()))
  }
}
