/// Closures
fn ex01() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let clousure_inferred = |i: i32| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", clousure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}

/// Capturing
fn ex02() {
    use std::mem;

    let color = String::from("green");
    let print = || println!("`color`: {}", color);
    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // call the closure using a mutable borrow.
    inc();

    // ^ TODO: try uncommenting this line
    inc();

    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // consume();
}

/// Using move before vertical pipes forces closure to take ownership of captured varibles
fn ex03() {
    let haystack = vec![1, 2, 3];

    let contains = move |neddle| haystack.contains(neddle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
}

pub fn run() {
    ex01();
    ex02();
    ex03();
}
