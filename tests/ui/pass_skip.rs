use numbered::Numbered;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Numbered)]
#[numbered(u8, no_display, no_variants)]
enum Kind {
    Process,
    File,
}

fn main() {
    assert_eq!(Kind::Process.number(), 0u8);
    assert_eq!(Kind::File.as_u8(), 1);
    assert_eq!(Kind::NUMBERS, &[0, 1]);
    assert_eq!(Kind::from_number(1).unwrap(), Kind::File);
    assert_eq!(u8::from(Kind::File), 1);
}
