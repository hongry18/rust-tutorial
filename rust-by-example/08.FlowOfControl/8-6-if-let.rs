fn ex01() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        },
        _ => {},
    }
}

fn ex02() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn ex03() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is boobar")
    }

    if let Foo::Qux(v) = c {
        println!("c is {}", v);
    }

    if let Foo::Qux(v @ 100) = c {
        println!("c is one hundered {}", v);
    }
}

/*
enum Foo4 {Bar}

fn ex04() {
    let a = Foo4::Bar;

    if Foo4::Bar == a {
        // this is causes a compile-time errror, Use `if let` instead
        println!("a is foobar");
    }
}
*/

fn main() {
    ex01();
    ex02();
    ex03();
}