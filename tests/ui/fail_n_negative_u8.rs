use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    #[numbered(n = -1)]
    Neg,
}

fn main() {}
