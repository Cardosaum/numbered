use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8, rename = "x")]
enum Mode {
    A,
}

fn main() {}
