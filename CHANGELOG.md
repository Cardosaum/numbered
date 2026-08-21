# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/Cardosaum/numbered/compare/numbered-v0.3.0...numbered-v0.3.1) - 2026-08-21

### Other

- Move number() onto Number, matching cognomen Label ([#6](https://github.com/Cardosaum/numbered/pull/6))

### Added

- `Number` and `FromNumber` in this crate

### Changed

- `number` / `as_*` are trait items, not inherent methods on `E`. Import
  `Number` or use UFCS
- `from_number` lives on `FromNumber` (fieldless only)
- `number` / `as_*` / `from_number` are no longer `const fn`. Trait
  methods cannot be. Const use goes through `Variants::NUMBERS`

## [0.3.0](https://github.com/Cardosaum/numbered/compare/numbered-v0.2.0...numbered-v0.3.0) - 2026-08-16

### Added

- number fielded variants without requiring Copy ([#4](https://github.com/Cardosaum/numbered/pull/4))

### Added

- fielded variants: `number(&self)` ignores the payload; `Variants::NUMBERS`
  and `COUNT` stay available; `VARIANTS` is `&[()]` of that length
- `Variants::COUNT` (`NUMBERS.len()`) and `Variants::Variant`

### Changed

- `number` / `as_*` take `&self` so non-`Copy` payloads work next to cognomen
- `from_number`, `TryFrom`, and serde `Deserialize` are omitted when any
  variant has a payload (a number cannot reconstruct fields)

## [0.2.0](https://github.com/Cardosaum/numbered/releases/tag/numbered-v0.2.0) - 2026-08-16

### Changed

- `VARIANTS` and `NUMBERS` live on the `Variants` trait, not as inherent items, so they cannot clash with another derive or user code
- Numbered no longer implements `Display`; print `e.number()`

### Fixed

- `TryFrom` now names `FromNumberError` instead of `Self::Error`, so a variant named `Error` compiles
- `Deserialize` now keeps the enum's generics, so `enum Flag<const N: usize>` compiles with `serde`

## [0.1.0](https://github.com/Cardosaum/numbered/releases/tag/numbered-v0.1.0) - 2026-08-16

### Added

- Initial release: derive stable integer numbers for unit-like enum variants
