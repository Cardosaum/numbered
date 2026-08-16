use numbered::Numbered;

#[derive(Numbered)]
#[numbered(u8)]
enum Collide {
    Zero,
    #[numbered(n = 0)]
    AlsoZero,
}

fn main() {}
