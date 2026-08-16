use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8, no_display, no_display)]
enum Mode {
    A,
}

fn main() {}
