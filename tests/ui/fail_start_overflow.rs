use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8, start = 256)]
enum Mode {
    A,
}

fn main() {}
