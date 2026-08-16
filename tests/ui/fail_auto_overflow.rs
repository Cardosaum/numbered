use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    A = 255,
    B,
}

fn main() {}
