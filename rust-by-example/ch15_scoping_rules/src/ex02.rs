/// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn ex01() {
    let x = 5u32;
    let y = x;

    println!("x is {}, y is {}", x, y);

    let a = Box::new(5i32);

    println!("a contains: {}", a);

    let b = a;

    // println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    destroy_box(b);

    // println!("b contains: {}", b);
}

/// Mutability
fn ex02() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

/// Partial moves
fn ex03() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    let Person { name, ref age } = person;
    println!("the person's age is {}", age);
    println!("the person's name is {}", name);

    println!("The person's age from person struct is {}", person.age);
}

pub fn run() {
    ex01();
    ex02();
    ex03();
}
