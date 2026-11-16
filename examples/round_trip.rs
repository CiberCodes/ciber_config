//! Demonstrates a round trip through the default CBR format.

use ciber_config::{FileFormat, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
  theme: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let settings = Settings { theme: "dark".into() };
  let format = FileFormat::default();
  let text = encode(&format, &settings)?;
  let _: Settings = decode(&format, &text)?;
  Ok(())
}
