# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/Cardosaum/numbered/compare/numbered-macros-v0.1.0...numbered-macros-v0.1.1) - 2026-08-16

### Other

- Put tables on Variants and stop implementing Display

### Changed

- emit `VARIANTS` / `NUMBERS` on `numbered::Variants`, not as inherent items
- do not implement `Display`

### Fixed

- `TryFrom` now names `FromNumberError` instead of `Self::Error`, so a variant named `Error` compiles

## [0.1.0](https://github.com/Cardosaum/numbered/releases/tag/numbered-macros-v0.1.0) - 2026-08-16

### Added

- Initial release: proc-macro implementation for numbered
