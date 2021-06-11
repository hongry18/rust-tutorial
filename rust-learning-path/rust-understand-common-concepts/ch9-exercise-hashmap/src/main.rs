use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();
    basket.insert(String::from("apple"), 1);
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("kiwi"), 3);

    basket
}

fn main() {
    let basket = fruit_basket();
    assert!(
        basket.len() >= 3,
        "basket must have at least three types of fruites"
    );

    assert!(
        basket.values().sum::<u32>() >= 5,
        "basket must have at least five fruites"
    );
}
