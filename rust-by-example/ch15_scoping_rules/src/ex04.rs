/// Lifetimes
fn ex00() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // let y: &'a i32 = &_x;
}

// Explicit annotation
fn ex01() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    failed_borrow();
}

fn ex02_print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

fn ex02_add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn ex02_print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

fn ex02_pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// Functions
fn ex02() {
    let x = 7;
    let y = 9;

    ex02_print_one(&x);
    ex02_print_multi(&x, &y);

    let z = ex02_pass_x(&x, &y);
    ex02_print_one(&z);

    let mut t = 3;
    ex02_add_one(&mut t);
    ex02_print_one(&t);
}

struct Ex03Owner(i32);

impl Ex03Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0)
    }
}

/// Methods
fn ex03() {
    let mut owner = Ex03Owner(18);

    owner.add_one();
    owner.print();
}

#[derive(Debug)]
struct Ex04Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct Ex04NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Ex04Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

/// structs
fn ex04() {
    let x = 18;
    let y = 15;

    let single = Ex04Borrowed(&x);
    let double = Ex04NamedBorrowed { x: &x, y: &y };
    let reference = Ex04Either::Ref(&x);
    let number = Ex04Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

#[derive(Debug)]
struct Ex05Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Ex05Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

// traits
fn ex05() {
    let b: Ex05Borrowed = Default::default();
    println!("b is {:?}", b);
}

use std::fmt::Debug;

#[derive(Debug)]
struct Ex06Ref<'a, T: 'a>(&'a T);

fn ex06_print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

fn ex06_print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

/// Bounds
fn ex06() {
    let x = 7;
    let ref_x = Ex06Ref(&x);
    ex06_print_ref(&x);
    ex06_print(ref_x);
}

fn ex07_multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

fn ex07_choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

/// Coercion
fn ex07() {
    let first = 2;

    {
        let second = 3;
        println!("The product is {}", ex07_multiply(&first, &second));
        println!("{} is the first", ex07_choose_first(&first, &second));
    }
}

static EX08NUM: i32 = 18;

fn ex08_coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &EX08NUM
}

/// Static
fn ex08() {
    {
        let static_string = "i'm in read-only memory";
        println!("static_string: {}", static_string);
    }

    {
        let lifetime_num = 0;
        let coerced_static = ex08_coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", EX08NUM);
}

fn ex08_1_print_it(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn ex08_1() {
    let i = 5;
    ex08_1_print_it(i);
    // ex08_1_print_it(&i);
}

fn ex09_elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn ex09_annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

fn ex09_elided_pass(x: &i32) -> &i32 {
    x
}

fn ex09_annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn ex09() {
    let x = 3;

    ex09_elided_input(&x);
    ex09_annotated_input(&x);

    println!("`elided_pass`: {}", ex09_elided_pass(&x));
    println!("`annotated_pass`: {}", ex09_annotated_pass(&x));
}

/// Lifetimes
pub fn run() {
    ex00();
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
    ex06();
    ex07();
    ex08();
    ex08_1();
    ex09();
}
