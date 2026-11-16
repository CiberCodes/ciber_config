# Feature flags

CBR support is always compiled and is the default selected by
`FileFormat::default()`. The other built-in formats are opt-in:

| Feature | Variant | Codec |
| --- | --- | --- |
| `json` | `FileFormat::Json` | JSON |
| `ron` | `FileFormat::Ron` | RON |
| `toml` | `FileFormat::Toml` | TOML |
| `yml` | `FileFormat::Yaml` | YAML |
| `custom_fmt` | `FileFormat::Other` | User implementation |
| `full` | All above | All built-in and custom support |

Enable one format in the manifest:

```toml
ciber_config = { version = "0.1", features = ["json"] }
```

Or with Cargo:

```sh
cargo add ciber_config --features json
```

Features can be combined. For example, `features = ["json", "toml"]` enables
both variants. `full` enables every listed format feature.
