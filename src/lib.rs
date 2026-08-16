//! Compile-time integer numbers for unit-like enum variants.
//!
//! This crate gives each variant a stable integer: a `u8` (or another chosen
//! repr) assigned at compile time. Conversion is a `match` on a literal.
//!
//! Downstream crates use this to drop handwritten `from_u8` / `as_u8` matches
//! on status codes, wire tags, and similar closed integer sets.
//!
//! # Quick start
//!
//! ```
//! use numbered::{Numbered, Variants};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
//! #[numbered(u8)]
//! enum Kind {
//!     Process,                 // 0
//!     File,                    // 1
//!     #[numbered(n = 10)]
//!     Network,                 // 10
//!     Socket,                  // 11  (continues after last explicit)
//! }
//!
//! assert_eq!(Kind::Process.number(), 0u8);
//! assert_eq!(Kind::File.as_u8(), 1);
//! assert_eq!(Kind::from_number(10).unwrap(), Kind::Network);
//! assert_eq!(Kind::try_from(11u8), Ok(Kind::Socket));
//! assert!(Kind::Process == 0u8);
//! assert_eq!(Kind::VARIANTS.len(), 4);
//! assert_eq!(Kind::NUMBERS, &[0, 1, 10, 11]);
//! ```
//!
//! Native Rust discriminants (`Process = 1`) are treated as
//! `#[numbered(n = 1)]`. If both are present they must agree.
//! A variant named `Error` is fine: generated `TryFrom` names
//! [`FromNumberError`] instead of `Self::Error`.
//!
//! # Attributes
//!
//! **Container** (required): `#[numbered(<repr>, ...)]`
//!
//! - A repr type: `u8`, `u16`, `u32`, `u64`, `usize`, `i8`, `i16`, `i32`,
//!   `i64`, or `isize`. First positional, or `repr = u8`.
//! - `start = 1`: first auto-assigned number when no discriminant / `n` is
//!   set (default `0`).
//! - `crate = ::other::numbered`: path used in generated code when this crate
//!   is re-exported under another name.
//!
//! **Variant** (optional): `#[numbered(n = 5)]`
//!
//! Pins that variant to an explicit integer. A native `= 5` discriminant
//! does the same. Gaps are allowed. After an explicit number, the next auto
//! variant is `last + 1`.
//!
//! ```
//! use numbered::Numbered;
//!
//! #[derive(Debug, PartialEq, Numbered)]
//! #[numbered(u8, start = 1)]
//! enum Wire {
//!     Open,                    // 1
//!     #[numbered(n = 10)]
//!     IoFailed,                // 10
//!     Closed = 20,             // 20
//!     Draining,                // 21
//! }
//!
//! assert_eq!(Wire::Open.number(), 1);
//! assert_eq!(Wire::IoFailed.as_u8(), 10);
//! assert_eq!(Wire::from_number(21).unwrap(), Wire::Draining);
//! ```
//!
//! Violations (non-enum, fields, empty enum, missing repr, duplicate keys,
//! unknown keys, collisions, overflow, disagreeing discriminant) are
//! compile errors.
//!
//! # Generated API
//!
//! For `#[numbered(u8)]` on `E`:
//!
//! - `number()` / `as_u8()` -> `u8` (`as_*` follows the repr)
//! - `from_number` / `from_u8` -> `Result<Self, FromNumberError<u8>>`
//! - [`Variants`] (non-generic enums): `E::VARIANTS` and `E::NUMBERS` after
//!   `use numbered::Variants`. These are trait items, so they cannot clash
//!   with another derive or a user `const VARIANTS`.
//! - No `Display` impl. Print the number with `e.number()`.
//! - `From<E> for u8`, `TryFrom<u8> for E`
//! - `PartialEq<u8>` / `PartialEq<E>` both ways
//! - `Serialize` / `Deserialize` (feature `serde`): the number, not a string
//!
//! # Features
//!
//! | Feature | Default | Unlocks |
//! |---------|---------|---------|
//! | `std` | yes | `alloc` + [`std::error::Error`] for [`FromNumberError`] |
//! | `alloc` | via `std` | kept for feature parity; the unmatched number is always stored |
//! | `serde` | no | `Serialize` / `Deserialize` |
//!
//! # `no_std`
//!
//! ```toml
//! numbered = { version = "0.2", default-features = false }
//! ```
//!
//! Numbers, parse, `From` / `TryFrom`, and [`Variants`] use only `core`.
//! Add `features = ["serde"]` for wire formats.
//!
//! # MSRV
//!
//! Rust 1.71.1.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate self as numbered;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc(inline)]
pub use numbered_macros::Numbered;

/// Declaration-order tables for a non-generic numbered enum.
///
/// `VARIANTS` and `NUMBERS` live on this trait, not as inherent items on
/// `E`. Another derive, or a user `const VARIANTS`, cannot clash with them.
/// Import the trait to use `E::VARIANTS`, or spell the path:
/// `<E as numbered::Variants>::VARIANTS`.
///
/// ```
/// use numbered::{Numbered, Variants};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Numbered)]
/// #[numbered(u8)]
/// enum Kind {
///     A,
///     B,
/// }
///
/// assert_eq!(Kind::VARIANTS, &[Kind::A, Kind::B]);
/// assert_eq!(Kind::NUMBERS, &[0, 1]);
/// ```
pub trait Variants: Sized + 'static {
    /// Integer type chosen in `#[numbered(<repr>)]`.
    type Repr: Copy;

    /// All variants in declaration order.
    const VARIANTS: &'static [Self];

    /// Assigned number for each variant in declaration order.
    const NUMBERS: &'static [Self::Repr];
}

/// Error returned when a number matches no declared variant.
///
/// Produced by [`TryFrom`](core::convert::TryFrom) and `E::from_number`.
/// The unmatched number is stored in [`Self::number`]. With `std`, this
/// implements [`std::error::Error`].
///
/// ```
/// use numbered::{FromNumberError, Numbered};
///
/// #[derive(Debug, Numbered)]
/// #[numbered(u8)]
/// enum Mode {
///     SingleProcess,
/// }
///
/// let err = Mode::from_number(9).unwrap_err();
/// assert_eq!(err.number, 9);
/// assert!(err.to_string().contains("9"));
/// let _: FromNumberError<u8> = err;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FromNumberError<T> {
    /// The number that did not match.
    pub number: T,
}

impl<T> FromNumberError<T> {
    /// Build an error for the unmatched `number`.
    #[must_use]
    pub const fn new(number: T) -> Self {
        Self { number }
    }
}

impl<T: core::fmt::Display> core::fmt::Display for FromNumberError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "no numbered variant has number {}", self.number)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<T: core::fmt::Debug + core::fmt::Display> std::error::Error for FromNumberError<T> {}

#[cfg(feature = "serde")]
#[doc(hidden)]
pub use serde as __serde;

#[cfg(test)]
mod tests {
    extern crate std;

    use std::string::ToString;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(u8)]
    enum Kind {
        Process,
        File,
        #[numbered(n = 10)]
        Network,
        Socket,
    }

    #[test]
    fn numbers_and_tables() {
        assert_eq!(Kind::Process.number(), 0u8);
        assert_eq!(Kind::File.as_u8(), 1);
        assert_eq!(Kind::from_number(10).unwrap(), Kind::Network);
        assert_eq!(Kind::try_from(11u8), Ok(Kind::Socket));
        assert!(Kind::Process == 0u8);
        assert!(0u8 == Kind::Process);
        assert_eq!(
            Kind::VARIANTS,
            &[Kind::Process, Kind::File, Kind::Network, Kind::Socket]
        );
        assert_eq!(Kind::NUMBERS, &[0, 1, 10, 11]);
        assert_eq!(Kind::Process.number().to_string(), "0");
        assert_eq!(u8::from(Kind::File), 1);
        assert_eq!(Kind::from_u8(11).unwrap(), Kind::Socket);
    }

    #[test]
    fn parse() {
        assert_eq!(Kind::from_u8(10).unwrap(), Kind::Network);
        assert!(Kind::from_number(99).is_err());
        let err = Kind::from_number(99).unwrap_err();
        assert_eq!(err.number, 99);
        assert!(err.to_string().contains("99"));
        #[cfg(feature = "std")]
        {
            let _: &dyn std::error::Error = &err;
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(i8, start = -1)]
    enum Signed {
        Neg,
        Zero,
        Pos = 5,
        Next,
    }

    #[test]
    fn signed_start_and_discriminant() {
        assert_eq!(Signed::Neg.number(), -1);
        assert_eq!(Signed::Zero.as_i8(), 0);
        assert_eq!(Signed::Pos.number(), 5);
        assert_eq!(Signed::Next.number(), 6);
        assert_eq!(Signed::from_number(-1).unwrap(), Signed::Neg);
        assert_eq!(Signed::NUMBERS, &[-1, 0, 5, 6]);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(u8)]
    enum Shared {
        A,
        B,
    }

    impl core::fmt::Display for Shared {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("user")
        }
    }

    impl Shared {
        pub const VARIANTS: &'static [&'static str] = &["a", "b"];
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(u8, start = 1)]
    enum Level {
        Error,
        Warn,
        Info,
        Debug,
    }

    #[test]
    fn error_variant_try_from() {
        assert_eq!(Level::Error.number(), 1);
        assert_eq!(Level::try_from(1u8), Ok(Level::Error));
        assert_eq!(Level::from_number(4).unwrap(), Level::Debug);
        assert!(Level::from_number(0).is_err());
        assert_eq!(Level::Error.number(), 1);
    }

    #[test]
    fn tables_do_not_clash_with_user_items() {
        assert_eq!(Shared::VARIANTS, &["a", "b"]);
        assert_eq!(<Shared as Variants>::VARIANTS, &[Shared::A, Shared::B]);
        assert_eq!(<Shared as Variants>::NUMBERS, &[0, 1]);
        assert_eq!(Shared::A.to_string(), "user");
        assert_eq!(Shared::A.number(), 0);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(repr = u16, start = 1)]
    enum Port {
        Http,
        Https = 443,
        Alt,
    }

    #[test]
    fn repr_key_and_start() {
        assert_eq!(Port::Http.as_u16(), 1);
        assert_eq!(Port::Https.number(), 443);
        assert_eq!(Port::Alt.number(), 444);
        assert_eq!(u16::from(Port::Http), 1);
        assert_eq!(Port::try_from(443u16), Ok(Port::Https));
    }

    const fn kind_number_in_const() -> u8 {
        Kind::Process.number()
    }

    #[test]
    fn works_in_const() {
        assert_eq!(kind_number_in_const(), 0);
        const FROM: Result<Kind, FromNumberError<u8>> = Kind::from_number(1);
        assert_eq!(FROM, Ok(Kind::File));
        const TABLES: &[u8] = Kind::NUMBERS;
        assert_eq!(TABLES, &[0, 1, 10, 11]);
    }

    #[test]
    fn from_number_error_new() {
        let err = FromNumberError::new(7u8);
        assert_eq!(err.number, 7);
        assert_eq!(err, Kind::from_number(7).unwrap_err());
        assert_eq!(err.to_string(), "no numbered variant has number 7");
    }

    #[test]
    fn partial_eq_mismatch() {
        assert!(Kind::Process != 1u8);
        assert!(1u8 != Kind::Process);
        assert!(Kind::File != 99u8);
        assert!(Kind::Network == 10u8);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(u8)]
    enum Literals {
        Hex = 0x10,
        #[numbered(n = 0x20)]
        AlsoHex,
        Next,
        #[numbered(n = 5)]
        Agree = 5,
        Paren = (7),
    }

    #[test]
    fn hex_paren_and_agreeing_discriminant() {
        assert_eq!(Literals::Hex.number(), 16);
        assert_eq!(Literals::AlsoHex.number(), 32);
        assert_eq!(Literals::Next.number(), 33);
        assert_eq!(Literals::Agree.number(), 5);
        assert_eq!(Literals::Paren.number(), 7);
        assert_eq!(Literals::from_number(16).unwrap(), Literals::Hex);
        assert_eq!(Literals::NUMBERS, &[16, 32, 33, 5, 7]);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(i8, start = -128)]
    enum I8Min {
        Lo,
        Next,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(u32)]
    enum Wide {
        A,
        B = 1_000,
        C,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(usize)]
    enum UsizeKind {
        A,
        B,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(isize)]
    enum IsizeKind {
        A = -1,
        B,
    }

    #[test]
    fn other_reprs() {
        assert_eq!(I8Min::Lo.as_i8(), -128);
        assert_eq!(I8Min::Next.number(), -127);
        assert_eq!(Wide::B.as_u32(), 1_000);
        assert_eq!(Wide::C.number(), 1_001);
        assert_eq!(UsizeKind::B.as_usize(), 1);
        assert_eq!(IsizeKind::A.as_isize(), -1);
        assert_eq!(IsizeKind::B.number(), 0);
        assert_eq!(u32::from(Wide::A), 0);
        assert_eq!(Wide::try_from(1_000u32), Ok(Wide::B));
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
    #[numbered(u8)]
    enum Generic<const N: usize> {
        A,
        B,
    }

    #[test]
    fn generic_has_numbers_not_tables() {
        assert_eq!(Generic::<3>::A.number(), 0);
        assert_eq!(Generic::<3>::from_number(1).unwrap(), Generic::<3>::B);
        assert_eq!(u8::from(Generic::<1>::A), 0);
    }

    impl Shared {
        pub const NUMBERS: &'static [&'static str] = &["zero", "one"];
    }

    #[test]
    fn user_numbers_do_not_hide_trait_numbers() {
        assert_eq!(Shared::NUMBERS, &["zero", "one"]);
        assert_eq!(<Shared as Variants>::NUMBERS, &[0, 1]);
    }

    #[test]
    fn signed_from_and_eq() {
        assert_eq!(i8::from(Signed::Neg), -1);
        assert_eq!(Signed::from_i8(5).unwrap(), Signed::Pos);
        assert!(Signed::Neg == -1i8);
        assert!(-1i8 == Signed::Neg);
        assert!(Signed::Pos != 0i8);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_roundtrip() {
        let v = Kind::Network;
        let s = serde_json::to_string(&v).unwrap();
        assert_eq!(s, "10");
        let back: Kind = serde_json::from_str(&s).unwrap();
        assert_eq!(back, v);
        assert!(serde_json::from_str::<Kind>("99").is_err());
        let process: Kind = serde_json::from_str("0").unwrap();
        assert_eq!(process, Kind::Process);
        let generic = Generic::<3>::B;
        let gs = serde_json::to_string(&generic).unwrap();
        assert_eq!(gs, "1");
        let gback: Generic<3> = serde_json::from_str(&gs).unwrap();
        assert_eq!(gback, generic);
    }
}
