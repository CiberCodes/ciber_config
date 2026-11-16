# Custom formats

Enable `custom_fmt` to add `FileFormat::Other`. Implement
`CustomConfigurationFormat` with methods that exchange `serde_value::Value`:
this interface is object-safe, so a codec can be stored in `Box<dyn ...>`.

```rust,ignore
use ciber_config::{CustomConfigurationFormat, FileFormat, Result};

#[derive(Debug)]
struct MyCodec;

impl CustomConfigurationFormat for MyCodec {
    fn decode_value(&self, input: &str) -> Result<serde_value::Value> {
        // Parse input with your format library and return its serde value.
        todo!()
    }

    fn encode_value(&self, value: &serde_value::Value) -> Result<String> {
        // Serialize the value with your format library.
        todo!()
    }
}

let format = FileFormat::Other(Box::new(MyCodec));
```

Run the complete working example with:

```sh
cargo run --example custom_format --features custom_fmt
```

The codec owns the conversion between its textual representation and
`serde_value::Value`. The crate converts that value to or from the caller's
Serde type.
