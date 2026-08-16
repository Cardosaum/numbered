# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `no_display` and `no_variants` so Numbered can share a type with another derive that emits `Display` or `VARIANTS`

### Fixed

- `TryFrom` now names `FromNumberError` instead of `Self::Error`, so a variant named `Error` compiles

## [0.1.0](https://github.com/Cardosaum/numbered/releases/tag/numbered-v0.1.0) - 2026-08-16

### Added

- Initial release: derive stable integer numbers for unit-like enum variants
