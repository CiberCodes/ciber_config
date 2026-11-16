//! YAML serialization support.
use serde::{Serialize, de::DeserializeOwned};

use crate::{ConfigError, ConfigurationFormat, Result};
/// YAML formatter.
#[derive(Debug, Default, Clone, Copy)]
pub struct YamlFormat;
impl ConfigurationFormat for YamlFormat {
  fn decode<T: DeserializeOwned>(content: &str) -> Result<T> {
    serde_saphyr::from_str(content)
      .map_err(|e| ConfigError::new("YAML", e.to_string()))
  }
  fn encode<T: Serialize>(value: &T) -> Result<String> {
    serde_saphyr::to_string(value)
      .map_err(|e| ConfigError::new("YAML", e.to_string()))
  }
}
