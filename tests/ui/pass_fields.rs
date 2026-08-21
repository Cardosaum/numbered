use numbered::{Number, Numbered, Variants};

#[derive(Debug, Clone, PartialEq, Eq, Numbered)]
#[numbered(u8, start = 1)]
enum HostError {
    Unsupported { capability: &'static str },
    OpenFailed { cause: String },
    BadRequest { why: &'static str },
    Io { status: String },
}

fn main() {
    let e = HostError::OpenFailed {
        cause: String::from("busy"),
    };
    assert_eq!(e.number(), 2);
    assert_eq!(e.as_u8(), 2);
    assert!(e == 2u8);
    assert_eq!(<HostError as Variants>::NUMBERS, &[1, 2, 3, 4]);
    assert_eq!(HostError::VARIANTS.len(), 4);
    assert_eq!(HostError::COUNT, 4);
}
