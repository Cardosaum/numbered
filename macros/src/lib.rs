//! Proc-macro implementation for [`numbered`](https://docs.rs/numbered).
//!
//! Depend on the `numbered` crate, not this one. This crate is the host
//! proc-macro; generated items use `core` so the runtime can be `no_std`.

mod numbered;

use proc_macro::TokenStream;

/// Derive stable integer numbers for an enum.
///
/// # Container attribute
///
/// ```ignore
/// #[derive(Numbered)]
/// #[numbered(u8, start = 0, crate = ::numbered)]
/// enum Kind { Process }
/// ```
///
/// - A repr type (required): `u8`, `u16`, `u32`, `u64`, `usize`, `i8`,
///   `i16`, `i32`, `i64`, or `isize`. First positional, or `repr = u8`.
/// - `start = N`: first auto-assigned number when no discriminant / `n` is
///   set. Default: `0`.
/// - `crate = ::path`: crate path emitted in generated code. Default:
///   `::numbered`. Set this when you re-export numbered from another crate.
///
/// # Variant attribute
///
/// ```ignore
/// #[numbered(n = 10)]
/// Network,
/// ```
///
/// Pins that variant to an explicit integer. A native `= 10` discriminant
/// does the same. If both are present they must agree.
///
/// # Generated items
///
/// All of these are trait impls. Nothing is inherent on `E`.
///
/// - `numbered::Number`: `number` / `as_u8` (`as_*` follows the repr)
/// - `numbered::FromNumber`: `from_number` / `from_u8` (fieldless enums)
/// - `numbered::Variants` (non-generic enums): `VARIANTS` / `NUMBERS` /
///   `COUNT`. Fielded enums use `Variant = ()` so `VARIANTS.len()` still
///   works next to cognomen extras
/// - `From<Self> for <repr>`, `TryFrom<repr> for Self`
/// - `PartialEq<repr>` both ways
/// - `Serialize` / `Deserialize` (feature `serde`)
///
/// See the [`numbered`](https://docs.rs/numbered) crate docs for assignment
/// rules, features, and `no_std`.
#[proc_macro_derive(Numbered, attributes(numbered))]
pub fn derive_numbered(input: TokenStream) -> TokenStream {
    numbered::derive(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
