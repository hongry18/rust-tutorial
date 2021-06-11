fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAAaaaaa!!!!");
    }

    println!("Some refresing {} is all I need.", beverage);
}

/// Panic
pub fn main() {
    drink("water");
    drink("lemonade");
}
