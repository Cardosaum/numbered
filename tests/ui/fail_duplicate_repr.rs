use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8, u16)]
enum Mode {
    A,
}

fn main() {}
