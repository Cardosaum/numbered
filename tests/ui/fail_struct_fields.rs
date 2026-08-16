use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    Unit,
    Named { x: u8 },
}

fn main() {}
