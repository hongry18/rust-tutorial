#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: String) {}

fn main() {
    main1();
    //main2();
    //main3();
    main4();
}

fn main1() {
    let x;
    {
        let y = 42;
        x = &y;
        println!("x: {}", x);
    }

    // println!("x: {}", x); // reference panic
}

fn main2() {
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);
}

fn main3() {
    // let magic3 = String::from("abracadabra!");
    // let reuslt;
    // {
    //     let magic4 = String::from("shazam!");
    //     result = longest_word(&magic3, &magic4);
    //     //println!("The longest magic word is {}", result);
    // }

    // println!("The longest magic word is {}", result);
}

fn main4() {
    let text = String::from("the quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}

fn main5() {
    let text = String::from("the quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    //erase(text);
    println!("{:?}", fox);
    println!("{:?}", dog);
}

/*
// panic code
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
