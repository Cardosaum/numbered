use numbered::{Numbered, Variants};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8, start = 1,)]
enum Kind {
    A,
    #[numbered(n = 10,)]
    B,
}

fn main() {
    assert_eq!(Kind::A.number(), 1);
    assert_eq!(Kind::B.number(), 10);
    assert_eq!(Kind::NUMBERS, &[1, 10]);
}
