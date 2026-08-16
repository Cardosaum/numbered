use numbered::{Numbered, Variants};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8)]
enum Kind {
    Process, // 0
    File,    // 1
    #[numbered(n = 10)]
    Network, // 10
    Socket,  // 11  (continues after last explicit)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(repr = i8)]
enum Signed {
    Neg = -2, // native discriminant
    Zero,     // -1
    #[numbered(n = 7)]
    Lucky = 7, // both n and discriminant agree
    Next,     // 8
}

fn main() {
    assert_eq!(Kind::Process.number(), 0u8);
    assert_eq!(Kind::File.as_u8(), 1);
    assert_eq!(Kind::from_number(10).unwrap(), Kind::Network);
    assert_eq!(Kind::try_from(11u8), Ok(Kind::Socket));
    assert!(Kind::Process == 0u8);
    assert_eq!(Kind::VARIANTS.len(), 4);
    assert_eq!(Kind::NUMBERS, &[0, 1, 10, 11]);

    assert_eq!(Signed::Neg.as_i8(), -2);
    assert_eq!(Signed::Zero.number(), -1);
    assert_eq!(Signed::Lucky.number(), 7);
    assert_eq!(Signed::Next.number(), 8);
    assert_eq!(Signed::from_i8(-2).unwrap(), Signed::Neg);
    assert_eq!(Signed::NUMBERS, &[-2, -1, 7, 8]);
    assert_eq!(Signed::Lucky.number(), 7);
}
