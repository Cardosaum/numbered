use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    #[numbered(n = 5)]
    A = 3,
}

fn main() {}
