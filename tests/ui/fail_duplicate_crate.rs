use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8, crate = ::numbered, crate = ::numbered)]
enum Mode {
    A,
}

fn main() {}
