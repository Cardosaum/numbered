use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    A = 256,
}

fn main() {}
