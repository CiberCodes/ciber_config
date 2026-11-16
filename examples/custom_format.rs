//! Shows how to implement and select an object-safe custom format.
//!
//! Run with `cargo run --example custom_format --features custom_fmt`.

use ciber_config::{
  ConfigError, CustomConfigurationFormat, FileFormat, Result, decode, encode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct JsonLikeFormat;

impl CustomConfigurationFormat for JsonLikeFormat {
  fn decode_value(&self, content: &str) -> Result<serde_value::Value> {
    serde_json::from_str(content)
      .map_err(|error| ConfigError::new("json-like", error.to_string()))
  }

  fn encode_value(&self, value: &serde_value::Value) -> Result<String> {
    serde_json::to_string(value)
      .map_err(|error| ConfigError::new("json-like", error.to_string()))
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Settings {
  theme: String,
  retries: u8,
}

fn main() -> Result<()> {
  let format = FileFormat::Other(Box::new(JsonLikeFormat));
  let original = Settings { theme: "dark".into(), retries: 3 };

  let text = encode(&format, &original)?;
  let decoded: Settings = decode(&format, &text)?;

  assert_eq!(decoded, original);
  println!("{text}");
  Ok(())
}
