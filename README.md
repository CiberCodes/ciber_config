# ciber_config

A Rust library for loading and managing application configuration.

> This crate is at an early stage. Its public API and supported configuration
> formats have not been defined yet.

## Status

The crate currently contains only its initial library scaffold. No configuration
loading functionality is available yet.

---

## Usage

Add the crate to your `Cargo.toml` once it is published:

```toml
[dependencies]
ciber_config = "0.1"
```

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for local development guidance and
[docs/README.md](docs/README.md) for the documentation index.

Run the local checks with:

```sh
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

## Examples and tests

The `examples/` and `tests/` directories are reserved for examples and
integration tests as public APIs are added.

---

## License

Licensed under either of the following, at your option:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

---

Maintained by [CiberCodes](https://github.com/CiberCodes).

