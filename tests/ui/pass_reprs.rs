use numbered::{Numbered, Variants};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u64)]
enum Wide {
    A,
    B = 1_000_000,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(i16, start = -2)]
enum Small {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(i32)]
enum Mid {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(i64)]
enum Big {
    A = -4,
    B,
}

fn main() {
    assert_eq!(Wide::B.as_u64(), 1_000_000);
    assert_eq!(Wide::C.number(), 1_000_001);
    assert_eq!(Wide::NUMBERS, &[0, 1_000_000, 1_000_001]);
    assert_eq!(Small::A.as_i16(), -2);
    assert_eq!(Small::B.number(), -1);
    assert_eq!(Mid::B.as_i32(), 1);
    assert_eq!(Big::A.as_i64(), -4);
    assert_eq!(Big::B.number(), -3);
    assert_eq!(i64::from(Big::A), -4);
}
