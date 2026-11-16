# Changelog

Notable changes to this project are recorded here.

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- CBR, RON, TOML, JSON, and YAML format support with opt-in Cargo features;
  CBR remains available by default.
- The `FileFormat` selector and public `encode`/`decode` API.
- Object-safe custom format support through `CustomConfigurationFormat` and
  `FileFormat::Other`, enabled by `custom_fmt`.
- Round-trip and invalid-input tests for built-in and custom codecs, plus usage
  examples.
- `Unicode-3.0` to the `deny.toml` license allowlist for the transitive
  `unicode-ident` dependency.

### Changed

- Replaced `serde_yaml` with `serde-saphyr` as the YAML backend.
- Renamed the public format operations from `parse`/`serialize` to `decode`/
  `encode`.

[Unreleased]: https://github.com/CiberCodes/ciber_config/compare/v0.1.0...HEAD
