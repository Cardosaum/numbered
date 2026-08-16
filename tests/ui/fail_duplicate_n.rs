use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    #[numbered(n = 1, n = 2)]
    A,
}

fn main() {}
