use numbered::{Numbered, Variants};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8)]
enum Kind {
    #[numbered(n = 10)]
    Network,
    #[numbered(n = 20)]
    Other,
}

fn main() {
    assert_eq!(Kind::Network.number(), 10);
    assert_eq!(Kind::Other.as_u8(), 20);
    assert_eq!(Kind::from_u8(10).unwrap(), Kind::Network);
    assert_eq!(Kind::NUMBERS, &[10, 20]);
    assert!(Kind::from_number(11).is_err());
}
