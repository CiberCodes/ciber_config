# Contributing

Thanks for considering a contribution. The crate is an early scaffold, so please
open an issue to discuss substantial API or format proposals before implementing
them.

## Development setup

Install a stable Rust toolchain and `cargo-deny`. Then run:

```sh
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo deny check
```

Keep changes focused, document public APIs, and add integration tests for new
observable behavior. Update `CHANGELOG.md` when a change is user-facing.
