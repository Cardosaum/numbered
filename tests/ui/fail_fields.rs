use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Mode {
    Unit,
    WithField(u8),
}

fn main() {}
