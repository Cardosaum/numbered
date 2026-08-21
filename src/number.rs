//! Number accessors live on [`Number`], not as inherent methods on the enum.

macro_rules! as_number_alias {
    ($($method:ident : $ty:ty),+ $(,)?) => {
        $(
            /// Alias of [`number`](Self::number) when [`Repr`](Self::Repr)
            #[doc = concat!(" is `", stringify!($ty), "`.")]
            #[inline]
            #[must_use]
            fn $method(&self) -> $ty
            where
                Self: Number<Repr = $ty>,
            {
                self.number()
            }
        )+
    };
}

macro_rules! from_number_alias {
    ($($method:ident : $ty:ty),+ $(,)?) => {
        $(
            /// Alias of [`from_number`](Self::from_number) when
            #[doc = concat!("[`Repr`](Self::Repr) is `", stringify!($ty), "`.")]
            #[inline]
            fn $method(n: $ty) -> Result<Self, crate::FromNumberError<$ty>>
            where
                Self: FromNumber<Repr = $ty>,
            {
                Self::from_number(n)
            }
        )+
    };
}

/// Stable integer number for a numbered enum.
///
/// Import this trait to call `e.number()` / `e.as_u8()`. A user inherent
/// `fn number` still compiles; use `<E as Number>::number(&e)`.
///
/// [`PartialEq`](core::cmp::PartialEq) on the enum (generated) compares
/// this number.
pub trait Number {
    /// Integer type chosen in `#[numbered(<repr>)]`.
    type Repr: Copy;

    /// Stable integer number assigned to this variant.
    ///
    /// Overridden by `#[numbered(n = ...)]` or a native discriminant.
    /// Takes `&self` so variants with a payload do not need `Copy`.
    #[must_use]
    fn number(&self) -> Self::Repr;

    as_number_alias! {
        as_u8: u8,
        as_u16: u16,
        as_u32: u32,
        as_u64: u64,
        as_usize: usize,
        as_i8: i8,
        as_i16: i16,
        as_i32: i32,
        as_i64: i64,
        as_isize: isize,
    }
}

/// Parse a number into `Self`.
///
/// Implemented only for fieldless enums: a number cannot rebuild a payload.
pub trait FromNumber: Sized {
    /// Integer type chosen in `#[numbered(<repr>)]`.
    type Repr: Copy;

    /// Parse a number into `Self`.
    ///
    /// Equivalent to [`TryFrom::try_from`](core::convert::TryFrom).
    fn from_number(n: Self::Repr) -> Result<Self, crate::FromNumberError<Self::Repr>>;

    from_number_alias! {
        from_u8: u8,
        from_u16: u16,
        from_u32: u32,
        from_u64: u64,
        from_usize: usize,
        from_i8: i8,
        from_i16: i16,
        from_i32: i32,
        from_i64: i64,
        from_isize: isize,
    }
}
