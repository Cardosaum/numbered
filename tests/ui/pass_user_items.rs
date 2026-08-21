use numbered::{Number, Numbered, Variants};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8)]
enum Kind {
    A,
    B,
}

impl core::fmt::Display for Kind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("user")
    }
}

impl Kind {
    pub const VARIANTS: &'static [&'static str] = &["a", "b"];
    pub const NUMBERS: &'static [&'static str] = &["zero", "one"];

    fn number(&self) -> u8 {
        9
    }

    fn as_u8(&self) -> u8 {
        8
    }
}

fn main() {
    assert_eq!(Kind::VARIANTS, &["a", "b"]);
    assert_eq!(Kind::NUMBERS, &["zero", "one"]);
    assert_eq!(<Kind as Variants>::VARIANTS, &[Kind::A, Kind::B]);
    assert_eq!(<Kind as Variants>::NUMBERS, &[0, 1]);
    assert_eq!(Kind::A.to_string(), "user");
    assert_eq!(Kind::A.number(), 9);
    assert_eq!(Kind::A.as_u8(), 8);
    assert_eq!(<Kind as Number>::number(&Kind::A), 0);
    assert_eq!(<Kind as Number>::as_u8(&Kind::A), 0);
}
