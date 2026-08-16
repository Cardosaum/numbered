use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8, no_variants, no_variants)]
enum Mode {
    A,
}

fn main() {}
