use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    A = 1 + 1,
}

fn main() {}
