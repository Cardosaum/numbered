# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- fielded variants and `Variants::Variant` / `COUNT` so tables work next to
  cognomen extras

### Changed

- `number` / `as_*` take `&self`
- omit `from_number` / `TryFrom` / serde `Deserialize` when a variant has
  a payload

## [0.2.0](https://github.com/Cardosaum/numbered/releases/tag/numbered-macros-v0.2.0) - 2026-08-16

### Changed

- emit `VARIANTS` / `NUMBERS` on `numbered::Variants`, not as inherent items
- do not implement `Display`

### Fixed

- `TryFrom` now names `FromNumberError` instead of `Self::Error`, so a variant named `Error` compiles
- `Deserialize` now keeps the enum's generics, so const-generic enums compile with `serde`

## [0.1.0](https://github.com/Cardosaum/numbered/releases/tag/numbered-macros-v0.1.0) - 2026-08-16

### Added

- Initial release: proc-macro implementation for numbered
