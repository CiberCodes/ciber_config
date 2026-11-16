# API and format selection

The crate provides `encode` and `decode` functions over Serde-compatible types.
Select a format with `FileFormat`; CBR is available without enabling a feature
and is the default.

```rust
use ciber_config::{decode, encode, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Settings {
    theme: String,
}

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let value = Settings { theme: "dark".into() };
let text = encode(&FileFormat::default(), &value)?;
let decoded: Settings = decode(&FileFormat::Cbr, &text)?;
assert_eq!(decoded, value);
# Ok(())
# }
```

When a format feature is enabled, its `FileFormat` variant can be passed to the
same functions. Each formatter returns `ConfigError` on parse or serialization
failure. See the crate README for an example covering all built-in formats.
