use numbered::{FromNumber, Number, Numbered};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(i8)]
enum Kind {
    A = -4,
    B,
    #[numbered(n = -1)]
    C,
}

fn main() {
    assert_eq!(Kind::A.number(), -4);
    assert_eq!(Kind::B.number(), -3);
    assert_eq!(Kind::C.as_i8(), -1);
    assert_eq!(Kind::from_i8(-4).unwrap(), Kind::A);
    assert_eq!(i8::from(Kind::C), -1);
}
