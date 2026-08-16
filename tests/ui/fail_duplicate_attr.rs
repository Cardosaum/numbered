use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
#[numbered(u16)]
enum Mode {
    A,
}

fn main() {}
