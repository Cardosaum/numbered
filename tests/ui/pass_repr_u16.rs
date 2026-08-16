use numbered::Numbered;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u16)]
enum Kind {
    A,
    B,
}

fn main() {
    assert_eq!(Kind::A.as_u16(), 0u16);
    assert_eq!(Kind::B.number(), 1);
    assert_eq!(Kind::from_u16(1).unwrap(), Kind::B);
    assert_eq!(u16::from(Kind::A), 0);
    assert_eq!(Kind::try_from(0u16), Ok(Kind::A));
    assert_eq!(Kind::NUMBERS, &[0u16, 1]);
}
