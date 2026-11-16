# Ciber Config

Configuration encoding and decoding for CBR, with optional RON, TOML, JSON, and
YAML formats.

## Features

| Cargo feature | Format/variant enabled |
| --- | --- |
| *(none)* | CBR only; `FileFormat::default()` is `Cbr` |
| `ron` | RON and `FileFormat::Ron` |
| `toml` | TOML and `FileFormat::Toml` |
| `json` | JSON and `FileFormat::Json` |
| `yml` | YAML and `FileFormat::Yaml` |
| `custom_fmt` | `FileFormat::Other` and custom format support |
| `full` | All built-in formats and custom formats |

Each format feature is optional. CBR is always available and is the default.

The dependency license checks in `deny.toml` allow MIT, Apache-2.0, and
Unicode-3.0. The Unicode license is required by the transitive `unicode-ident`
dependency; `cargo deny check` verifies this allowlist and the dependency
advisories, bans, and sources.

## Enable formats

In `Cargo.toml`, enable only the formats the application uses:

```toml
[dependencies]
ciber_config = { version = "0.1", features = ["json", "toml"] }
serde = { version = "1", features = ["derive"] }
```

Or use Cargo commands:

```sh
cargo add ciber_config --features json
cargo add ciber_config --features ron
cargo add ciber_config --features toml
cargo add ciber_config --features yml
cargo add ciber_config --features custom_fmt
cargo add ciber_config --features full
```

You can enable several features at once, for example:

```sh
cargo add ciber_config --features json,ron,yml
```

## Encode and decode formats

The same API is used for every format. The selected variant is available only
when its feature is enabled.

```rust
use ciber_config::{decode, encode, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Settings {
    theme: String,
    retries: u8,
}

# fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
let settings = Settings { theme: "dark".into(), retries: 3 };

// CBR needs no feature and is the default.
let cbr_text = encode(&FileFormat::default(), &settings)?;
let from_cbr: Settings = decode(&FileFormat::Cbr, &cbr_text)?;

// Enable the matching Cargo feature to select each format.
#[cfg(feature = "json")]
let from_json: Settings = {
    let text = encode(&FileFormat::Json, &settings)?;
    decode(&FileFormat::Json, &text)?
};
#[cfg(feature = "ron")]
let from_ron: Settings = {
    let text = encode(&FileFormat::Ron, &settings)?;
    decode(&FileFormat::Ron, &text)?
};
#[cfg(feature = "toml")]
let from_toml: Settings = {
    let text = encode(&FileFormat::Toml, &settings)?;
    decode(&FileFormat::Toml, &text)?
};
#[cfg(feature = "yml")]
let from_yaml: Settings = {
    let text = encode(&FileFormat::Yaml, &settings)?;
    decode(&FileFormat::Yaml, &text)?
};

let _ = from_cbr;
#[cfg(feature = "json")]
let _ = from_json;
#[cfg(feature = "ron")]
let _ = from_ron;
#[cfg(feature = "toml")]
let _ = from_toml;
#[cfg(feature = "yml")]
let _ = from_yaml;
Ok(())
}
```

Run the example using a selected format with its feature enabled, for example:

```sh
cargo run --example round_trip
cargo test --features json
```

## Custom format

Enable `custom_fmt` to add a custom codec. Its object-safe trait methods work
with `serde_value::Value`, while `FileFormat::Other` carries the codec:

```rust
#[cfg(feature = "custom_fmt")]
use ciber_config::{
    ConfigError, CustomConfigurationFormat, FileFormat, Result, decode, encode,
};
#[cfg(feature = "custom_fmt")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "custom_fmt")]
#[derive(Debug)]
struct JsonFormat;

#[cfg(feature = "custom_fmt")]
impl CustomConfigurationFormat for JsonFormat {
    fn decode_value(&self, input: &str) -> Result<serde_value::Value> {
        serde_json::from_str(input)
            .map_err(|error| ConfigError::new("json", error.to_string()))
    }

    fn encode_value(&self, value: &serde_value::Value) -> Result<String> {
        serde_json::to_string(value)
            .map_err(|error| ConfigError::new("json", error.to_string()))
    }
}

#[cfg(feature = "custom_fmt")]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Settings {
    theme: String,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
#[cfg(feature = "custom_fmt")]
{
let format = FileFormat::Other(Box::new(JsonFormat));
let settings = Settings { theme: "dark".into() };
let text = encode(&format, &settings)?;
let decoded: Settings = decode(&format, &text)?;
assert_eq!(decoded, settings);
}
Ok(())
}
```

A complete runnable example is available at
[`examples/custom_format.rs`](https://github.com/CiberCodes/ciber_config/blob/main/examples/custom_format.rs):

```sh
cargo run --example custom_format --features custom_fmt
```

See [CHANGELOG](https://github.com/CiberCodes/ciber_config/blob/main/CHANGELOG.md), [contribution guide](https://github.com/CiberCodes/ciber_config/blob/main/CONTRIBUTING.md), and
[docs index](https://github.com/CiberCodes/ciber_config/tree/main/docs).
