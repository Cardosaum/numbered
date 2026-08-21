use numbered::{FromNumber, Number, Numbered, Variants};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8)]
enum Kind {
    Process,
    File,
    Socket,
}

fn main() {
    assert_eq!(Kind::Process.number(), 0u8);
    assert_eq!(Kind::File.as_u8(), 1);
    assert_eq!(Kind::Socket.number(), 2);
    assert_eq!(Kind::from_number(1).unwrap(), Kind::File);
    assert_eq!(Kind::try_from(2u8), Ok(Kind::Socket));
    assert!(Kind::Process == 0u8);
    assert!(1u8 == Kind::File);
    assert_eq!(Kind::VARIANTS.len(), 3);
    assert_eq!(Kind::NUMBERS, &[0, 1, 2]);
    assert_eq!(Kind::Process.number(), 0);
    assert_eq!(u8::from(Kind::Socket), 2);
}
