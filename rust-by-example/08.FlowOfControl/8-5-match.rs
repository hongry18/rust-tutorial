fn ex01() {
    let number = 13;

    println!("Tell me about{}", number);
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

fn ex02() {
    // tuples
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is '0', 'y' is {:?}, and 'z' is {:?}", y, z),
        (1, ..) => println!("First is '1' and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn ex03() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    match color {
        Color::Red => println!("the color is red!"),
        Color::Blue => println!("the color is blue!"),
        Color::Green => println!("the color is green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, ligntness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }
}

fn ex04() {
    // ponters/ref
    let reference = &4;

    match reference {
        &val => println!("Gott a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Gott a value via destructuring: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("we added 10. `mut_value`: {:?}", m);
        }
    }
}

fn ex05() {
    struct Foo {
        x: (u32, u32),
        y: u32
    }

    let foo = Foo {x:(1,2), y:3};
    match foo {
        Foo {x: (1,b), y} => println!("First of x is 1, b={}, y={}", b, y),
        Foo {y:2, x:i} => println!("y is 2, i={:?}", i),
        Foo {y,..} => println!("y = {}, we don't care about x", y),
    }
}

fn ex06() {
    // guards
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x +y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first on is odd"),
        _ => println!("No correlation..."),
    }

    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i>0 => println!("Greater than zero"),
        _ => println!("Fell through"),
    }
}

fn age() -> u32 {
    15
}

fn ex07() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}",n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn ex08() {
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}

fn main() {
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
    ex06();
    ex07();
    ex08();
}