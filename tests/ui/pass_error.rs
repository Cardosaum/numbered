use numbered::Numbered;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8, start = 1)]
enum Level {
    Error,
    Warn,
    Info,
    Debug,
}

fn main() {
    assert_eq!(Level::Error.number(), 1);
    assert_eq!(Level::try_from(1u8), Ok(Level::Error));
    assert_eq!(Level::from_number(4).unwrap(), Level::Debug);
    assert!(Level::from_number(0).is_err());
    assert_eq!(Level::Error.to_string(), "1");
}
