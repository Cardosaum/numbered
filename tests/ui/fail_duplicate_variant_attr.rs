use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    #[numbered(n = 1)]
    #[numbered(n = 1)]
    A,
}

fn main() {}
