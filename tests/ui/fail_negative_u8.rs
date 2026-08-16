use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8, start = -1)]
enum Mode {
    A,
}

fn main() {}
