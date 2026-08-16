use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    #[numbered(foo = 1)]
    A,
}

fn main() {}
