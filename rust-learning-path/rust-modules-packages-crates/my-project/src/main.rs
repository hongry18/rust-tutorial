mod math {
    type Complex = (f64, f64);
    pub fn sin(f: f64) -> f64 {
        f
    }
    pub fn cos(f: f64) -> f64 {
        f
    }
    pub fn tan(f: f64) -> f64 {
        f
    }
}

struct Foo;

pub struct Bar {
    field: i32,
}

pub enum State {
    PubliclyAccessibleVariant,
    PubliclyAccessibleVariant2,
}

fn main() {
    println!("{}", math::cos(45.0));
}
