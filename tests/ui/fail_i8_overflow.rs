use numbered::Numbered;

#[derive(Numbered)]
#[numbered(i8)]
enum Mode {
    #[numbered(n = 128)]
    TooBig,
}

fn main() {}
