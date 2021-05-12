use std::marker::PhantomData;
use std::ops::Add;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

/// Phantom type parameters
fn ex01() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    println!("tuple 1: {:?} {:?}", _tuple1.0, _tuple1.1);
    println!("tuple 2: {:?} {:?}", _tuple2.0, _tuple2.1);
    println!("struct 1: {:?} {:?}", _struct1.first, _struct1.phantom);
    println!("struct 2: {:?} {:?}", _struct2.first, _struct2.phantom);
}

/// Create void enumerations to define unit types.
#[derive(Debug, Clone, Copy)]
enum Inch {}
/// Create void enumerations to define unit types.
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` is a type with phantom type parameter `Unit`,
/// and is not generic over the length type (that is `f64`).
///
/// `f64` already implements the `Clone` and `Copy` traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

/// Testcase: unit clarification
fn ex02() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Inch> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one foot = {:?} in", two_feet.0);
    println!("one meter + one meter = {:?} mm", two_meters.0);
}

pub fn run() {
    ex01();
    ex02();
}
