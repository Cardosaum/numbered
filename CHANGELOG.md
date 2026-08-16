# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- `VARIANTS` and `NUMBERS` live on the `Variants` trait, not as inherent items, so they cannot clash with another derive or user code
- Numbered no longer implements `Display`; print `e.number()`

### Fixed

- `TryFrom` now names `FromNumberError` instead of `Self::Error`, so a variant named `Error` compiles
- `Deserialize` now keeps the enum's generics, so `enum Flag<const N: usize>` compiles with `serde`

## [0.1.0](https://github.com/Cardosaum/numbered/releases/tag/numbered-v0.1.0) - 2026-08-16

### Added

- Initial release: derive stable integer numbers for unit-like enum variants
