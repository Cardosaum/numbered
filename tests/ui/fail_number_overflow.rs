use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    #[numbered(n = 256)]
    TooBig,
}

fn main() {}
