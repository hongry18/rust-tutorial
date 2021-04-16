use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

//let my_str = "hello";
//let my_string = String::from(my_str);

fn main() {
    let num = Number::from(30);
    println!("my number is {:?}", num);

    let int = 5;
    let num2: Number = int.into();
    println!("my number is {:?}", num2);
}
