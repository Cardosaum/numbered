use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
struct NotAnEnum {
    x: u8,
}

fn main() {}
