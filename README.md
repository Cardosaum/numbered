# numbered

[![crates.io](https://img.shields.io/crates/v/numbered.svg)](https://crates.io/crates/numbered)
[![docs.rs](https://docs.rs/numbered/badge.svg)](https://docs.rs/numbered)
[![license](https://img.shields.io/crates/l/numbered.svg)](https://github.com/Cardosaum/numbered)

Derive **stable integer numbers** for unit-like enum variants. This crate
kills the boilerplate of handwritten `from_u8` / `as_u8` matches: you pick a
repr, optionally pin a few variants, and the rest continue from the last
assigned number.

Full API reference: <https://docs.rs/numbered>

```toml
[dependencies]
numbered = "0.1"
```

```rust
use numbered::Numbered;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8)]
enum Kind {
    Process,                 // 0
    File,                    // 1
    #[numbered(n = 10)]
    Network,                 // 10
    Socket,                  // 11  (continues after last explicit)
}

assert_eq!(Kind::Process.number(), 0u8);
assert_eq!(Kind::File.as_u8(), 1);
assert_eq!(Kind::from_number(10).unwrap(), Kind::Network);
assert_eq!(Kind::try_from(11u8), Ok(Kind::Socket));
assert!(Kind::Process == 0u8);
assert_eq!(Kind::VARIANTS.len(), 4);
assert_eq!(Kind::NUMBERS, &[0, 1, 10, 11]);
```

Native Rust discriminants (`Process = 1`) are treated as `#[numbered(n = 1)]`.
If both are present they must agree, or the derive is a compile error. A
variant named `Error` is fine: generated `TryFrom` names `FromNumberError`
instead of `Self::Error`.

## Attributes

**Container** (required): `#[numbered(<repr>, ...)]`

| key | required | notes |
|-----|----------|-------|
| `u8` / `u16` / `u32` / `u64` / `usize` / `i8` / `i16` / `i32` / `i64` / `isize` | yes | first positional, or `repr = u8` |
| `start = 1` | no | first auto-assigned number when no discriminant / `n` is set (default `0`) |
| `crate = ::other::numbered` | no | path used in generated code when this crate is re-exported |
| `no_display` | no | skip `Display` so another derive (for example cognomen) can implement it |
| `no_variants` | no | skip `E::VARIANTS` (`NUMBERS` is still emitted) |

**Variant** (optional): `#[numbered(n = 5)]`

Pins that variant to an explicit integer. A native `= 5` discriminant does
the same. Gaps are allowed. After an explicit number, the next auto variant
is `last + 1`.

```rust
use numbered::Numbered;

#[derive(Debug, PartialEq, Numbered)]
#[numbered(u8, start = 1)]
enum Wire {
    Open,                    // 1
    #[numbered(n = 10)]
    IoFailed,                // 10
    Closed = 20,             // 20
    Draining,                // 21
}

assert_eq!(Wire::Open.number(), 1);
assert_eq!(Wire::IoFailed.as_u8(), 10);
assert_eq!(Wire::from_number(21).unwrap(), Wire::Draining);
```

Violations (non-enum, fields, empty enum, missing repr, duplicate keys,
unknown keys, collisions, overflow, disagreeing discriminant) are compile
errors, pinned by trybuild tests under `tests/ui/`.

## Generated API

For `#[numbered(u8)]` on `E`:

| item | notes |
|------|--------|
| `number()` / `as_u8()` | assigned number (`as_*` follows the repr: `as_u16`, `as_i32`, ...) |
| `from_number` / `from_u8` | `Result<Self, FromNumberError<u8>>` |
| `E::VARIANTS`, `E::NUMBERS` | declaration order (non-generic enums; `no_variants` skips `VARIANTS`) |
| `Display` | decimal number (`no_display` skips this) |
| `From<E> for u8`, `TryFrom<u8> for E` | always; uses `core` |
| `PartialEq<u8>` both ways | compare a variant against its number |
| `Serialize` / `Deserialize` | feature `serde`; the number, not a string |

## Features

| feature | default | unlocks |
|---------|---------|---------|
| `std` | yes | `alloc` + `std::error::Error` for `FromNumberError` |
| `alloc` | via `std` | kept for feature parity; the unmatched number is always stored |
| `serde` | no | `Serialize` / `Deserialize` as the number |

`no_std`, including embedded:

```toml
numbered = { version = "0.1", default-features = false }
```

Numbers, parse, `Display`, `From` / `TryFrom`, and `VARIANTS` use only
`core`. Add `features = ["serde"]` for wire formats.

## MSRV

Rust **1.71.1**. Floor is the pinned `proc-macro2` (rustc 1.71+).

## Publishing

`numbered` depends on `numbered-macros` by version. The first crates.io
upload must publish `numbered-macros` first, then `numbered`. New crate
names cannot use trusted publishing; that first upload is manual. Later
releases are prepared and published by release-plz on `main`.

## License

Dual-licensed under **MIT OR Apache-2.0**, see
[`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).
Copyright (c) 2026 Matheus Cardoso.
