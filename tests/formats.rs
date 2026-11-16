//! Integration tests for the built-in configuration formats.
#![allow(clippy::expect_used)]

use ciber_config::{FileFormat, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Settings {
  name: String,
  enabled: bool,
}

#[test]
fn cbr_is_the_default_format() {
  let value = Settings { name: "sample".into(), enabled: true };
  let format = FileFormat::default();
  let text = encode(&format, &value).expect("encode CBR");
  let decoded: Settings = decode(&format, &text).expect("decode CBR");
  assert_eq!(decoded, value);
}

#[cfg(feature = "json")]
#[test]
fn json_round_trip() { round_trip(&FileFormat::Json); }
#[cfg(feature = "ron")]
#[test]
fn ron_round_trip() { round_trip(&FileFormat::Ron); }
#[cfg(feature = "toml")]
#[test]
fn toml_round_trip() { round_trip(&FileFormat::Toml); }
#[cfg(feature = "yml")]
#[test]
fn yaml_round_trip() { round_trip(&FileFormat::Yaml); }

#[cfg(any(
  feature = "json",
  feature = "ron",
  feature = "toml",
  feature = "yml"
))]
fn round_trip(format: &FileFormat) {
  let value = Settings { name: "sample".into(), enabled: true };
  let text = encode(format, &value).expect("encode");
  let decoded: Settings = decode(format, &text).expect("decode");
  assert_eq!(decoded, value);
}

#[cfg(feature = "custom_fmt")]
#[derive(Debug)]
struct ExampleCustomFormat;

#[cfg(feature = "custom_fmt")]
impl ciber_config::CustomConfigurationFormat for ExampleCustomFormat {
  fn decode_value(
    &self,
    content: &str,
  ) -> ciber_config::Result<serde_value::Value> {
    serde_json::from_str(content).map_err(|error| {
      ciber_config::ConfigError::new("example-json", error.to_string())
    })
  }

  fn encode_value(
    &self,
    value: &serde_value::Value,
  ) -> ciber_config::Result<String> {
    serde_json::to_string(value).map_err(|error| {
      ciber_config::ConfigError::new("example-json", error.to_string())
    })
  }
}

#[cfg(feature = "custom_fmt")]
#[test]
fn custom_format_round_trip() {
  let format = FileFormat::Other(Box::new(ExampleCustomFormat));
  let original = Settings { name: "custom".into(), enabled: true };

  let encoded = encode(&format, &original).expect("encode custom format");
  let decoded: Settings =
    decode(&format, &encoded).expect("decode custom format");

  assert_eq!(decoded, original);
}

#[cfg(feature = "custom_fmt")]
#[test]
fn custom_format_reports_decode_errors() {
  let format = FileFormat::Other(Box::new(ExampleCustomFormat));
  let error = decode::<Settings>(&format, "not valid json")
    .expect_err("invalid custom input should fail");

  assert_eq!(error.format(), "example-json");
}
