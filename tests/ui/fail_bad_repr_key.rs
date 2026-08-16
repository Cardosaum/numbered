use numbered::Numbered;

#[derive(Numbered)]
#[numbered(repr = f32)]
enum Mode {
    A,
}

fn main() {}
