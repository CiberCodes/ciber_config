#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod error;
pub mod fmt;
mod format;

pub use error::{ConfigError, Result};
#[cfg(feature = "custom_fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "custom_fmt")))]
pub use format::CustomConfigurationFormat;
pub use format::{ConfigurationFormat, FileFormat};
use serde::{Serialize, de::DeserializeOwned};

/// Parses configuration text using the selected format.
///
/// # Errors
/// Returns an error if parsing fails or the result cannot be represented as `T`.
pub fn decode<T: DeserializeOwned>(
  format: &FileFormat,
  content: &str,
) -> Result<T> {
  match format {
    FileFormat::Cbr => fmt::cbr::CbrFormat::decode(content),
    #[cfg(feature = "ron")]
    FileFormat::Ron => fmt::ron::RonFormat::decode(content),
    #[cfg(feature = "toml")]
    FileFormat::Toml => fmt::toml::TomlFormat::decode(content),
    #[cfg(feature = "json")]
    FileFormat::Json => fmt::json::JsonFormat::decode(content),
    #[cfg(feature = "yml")]
    FileFormat::Yaml => fmt::yml::YamlFormat::decode(content),
    #[cfg(feature = "custom_fmt")]
    FileFormat::Other(codec) => {
      let value = codec.decode_value(content)?;
      T::deserialize(value)
        .map_err(|e| ConfigError::new("custom", e.to_string()))
    }
  }
}

/// Serializes a value using the selected format.
///
/// # Errors
/// Returns an error if the value cannot be represented by the selected format.
pub fn encode<T: Serialize>(format: &FileFormat, value: &T) -> Result<String> {
  match format {
    FileFormat::Cbr => fmt::cbr::CbrFormat::encode(value),
    #[cfg(feature = "ron")]
    FileFormat::Ron => fmt::ron::RonFormat::encode(value),
    #[cfg(feature = "toml")]
    FileFormat::Toml => fmt::toml::TomlFormat::encode(value),
    #[cfg(feature = "json")]
    FileFormat::Json => fmt::json::JsonFormat::encode(value),
    #[cfg(feature = "yml")]
    FileFormat::Yaml => fmt::yml::YamlFormat::encode(value),
    #[cfg(feature = "custom_fmt")]
    FileFormat::Other(codec) => {
      let value = serde_value::to_value(value)
        .map_err(|e| ConfigError::new("custom", e.to_string()))?;
      codec.encode_value(&value)
    }
  }
}
