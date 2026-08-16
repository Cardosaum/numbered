use numbered::{Numbered, Variants};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8, crate = ::numbered)]
enum Kind {
    A,
    B,
}

fn main() {
    assert_eq!(Kind::A.number(), 0);
    assert_eq!(Kind::from_number(1).unwrap(), Kind::B);
    assert_eq!(Kind::NUMBERS, &[0, 1]);
}
