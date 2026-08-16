use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8, start = 1, start = 2)]
enum Mode {
    A,
}

fn main() {}
