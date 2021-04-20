//use std::option::Option;

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    let first = fruits.get(0);
    println!("{:?}", first);

    let third = fruits.get(2);
    println!("{:?}", third);

    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    // pattern match
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Cocunuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    // if let 식
    let some_number: Option<u8> = Option::Some(7);
    match some_number {
        Option::Some(7) => println!("That's my lucky number!"),
        _ => {},
    }

    // match 와 동일한 동작
    if let Option::Some(7) = some_number {
        println!("That's my lucky number!");
    }

    let none_item: Option<&str> = Option::None;

    if let Option::None = none_item {
        println!("is none!");
    }

    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    // let empty_gift: Option<&str> = None;
    // assert_eq!(empty_gift.unwrap(), "candy"); // this will panic

    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    // let b: Option<&str> = None;
    // b.expect("friutes are healthy"); // this will panic

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}
