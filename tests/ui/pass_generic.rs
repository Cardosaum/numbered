use numbered::{FromNumber, Number, Numbered};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8)]
enum Flag<const N: usize> {
    Off,
    On,
}

fn main() {
    assert_eq!(Flag::<1>::Off.number(), 0);
    assert_eq!(Flag::<7>::On.as_u8(), 1);
    assert_eq!(Flag::<1>::from_number(1).unwrap(), Flag::<1>::On);
    assert_eq!(u8::from(Flag::<2>::Off), 0);
}
