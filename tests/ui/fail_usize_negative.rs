use numbered::Numbered;

#[derive(Numbered)]
#[numbered(usize, start = -1)]
enum Mode {
    A,
}

fn main() {}
