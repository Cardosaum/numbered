use numbered::{Numbered, Variants};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8, start = 1)]
enum Kind {
    A,
    B,
    C,
}

fn main() {
    assert_eq!(Kind::A.number(), 1);
    assert_eq!(Kind::B.as_u8(), 2);
    assert_eq!(Kind::C.number(), 3);
    assert_eq!(Kind::from_number(1).unwrap(), Kind::A);
    assert_eq!(Kind::NUMBERS, &[1, 2, 3]);
    assert!(Kind::from_number(0).is_err());
}
