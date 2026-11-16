//! Traits for built-in and user-defined configuration formats.

use serde::{Serialize, de::DeserializeOwned};

use crate::Result;

/// A built-in format capable of parsing and serializing typed values.
pub trait ConfigurationFormat: Send + Sync + 'static {
  /// Parses text into a value of type `T`.
  ///
  /// # Errors
  /// Returns an error if the text cannot be parsed as `T`.
  fn decode<T: DeserializeOwned>(content: &str) -> Result<T>;

  /// Serializes a value to text.
  ///
  /// # Errors
  /// Returns an error if the value cannot be represented by this format.
  fn encode<T: Serialize>(value: &T) -> Result<String>;
}

/// Object-safe interface for custom formats selected with [`FileFormat::Other`].
///
/// Custom formats exchange `serde_value::Value`, allowing type-erased dispatch
/// without generic methods. Implement this trait directly for a custom codec.
#[cfg(feature = "custom_fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "custom_fmt")))]
pub trait CustomConfigurationFormat: Send + Sync + std::fmt::Debug {
  /// Parses custom text into a dynamically typed serde value.
  ///
  /// # Errors
  /// Returns an error if the text is invalid for this custom format.
  fn decode_value(&self, content: &str) -> Result<serde_value::Value>;

  /// Serializes a dynamically typed serde value to custom text.
  ///
  /// # Errors
  /// Returns an error if the value cannot be represented by this custom format.
  fn encode_value(&self, value: &serde_value::Value) -> Result<String>;
}

/// Selects a built-in format or, with `custom_fmt`, a user-provided format.
#[derive(Default)]
pub enum FileFormat {
  /// CBR, also the default format.
  #[default]
  Cbr,
  /// RON format. Requires the `ron` feature.
  #[cfg(feature = "ron")]
  #[cfg_attr(docsrs, doc(cfg(feature = "ron")))]
  Ron,
  /// TOML format. Requires the `toml` feature.
  #[cfg(feature = "toml")]
  #[cfg_attr(docsrs, doc(cfg(feature = "toml")))]
  Toml,
  /// JSON format. Requires the `json` feature.
  #[cfg(feature = "json")]
  #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
  Json,
  /// YAML format. Requires the `yml` feature.
  #[cfg(feature = "yml")]
  #[cfg_attr(docsrs, doc(cfg(feature = "yml")))]
  Yaml,
  /// Custom format. Requires the `custom_fmt` feature.
  #[cfg(feature = "custom_fmt")]
  #[cfg_attr(docsrs, doc(cfg(feature = "custom_fmt")))]
  Other(Box<dyn CustomConfigurationFormat>),
}

impl std::fmt::Debug for FileFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Cbr => f.write_str("Cbr"),
      #[cfg(feature = "ron")]
      Self::Ron => f.write_str("Ron"),
      #[cfg(feature = "toml")]
      Self::Toml => f.write_str("Toml"),
      #[cfg(feature = "json")]
      Self::Json => f.write_str("Json"),
      #[cfg(feature = "yml")]
      Self::Yaml => f.write_str("Yaml"),
      #[cfg(feature = "custom_fmt")]
      Self::Other(codec) => f.debug_tuple("Other").field(codec).finish(),
    }
  }
}
