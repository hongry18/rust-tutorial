macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

fn ex01() {
    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    })
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

fn ex02() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4);
    test!(true; or false);
}

macro_rules! find_min {
    ($x: expr) => ($x);

    ($x: expr, $($y:expr), +) => (
        std::cmp::min($x, find_min!($($y), +))
    )
}

fn ex03() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2));
    println!("{}", find_min!(5, 5u32 * 3, 4u32));
}

pub fn main() {
    ex01();
    ex02();
    ex03();
}
