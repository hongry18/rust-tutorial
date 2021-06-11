use std::error;
use std::error::Error as _;
use std::fmt;
use std::num::ParseIntError;

fn double_first0(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // gen err 1
    2 * first.parse::<i32>().unwrap() // gen err 2
}

fn ex00() {
    let nums = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first0(nums));
    println!("The first doubled is {}", double_first0(empty));
    println!("The first doubled is {}", double_first0(strings));
}

fn double_first1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|f| f.parse::<i32>().map(|n| n * 2))
}

fn ex01() {
    let nums = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first1(nums));
    println!("The first doubled is {:?}", double_first1(empty));
    println!("The first doubled is {:?}", double_first1(strings));
}

fn double_first1_1(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|f| f.parse::<i32>().map(|n| n * 2));

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn ex01_1() {
    let nums = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first1_1(nums));
    println!("The first doubled is {:?}", double_first1_1(empty));
    println!("The first doubled is {:?}", double_first1_1(strings));
}

type Result2<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first2(v: Vec<&str>) -> Result2<i32> {
    v.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| i * 2))
}

fn print2(r: Result2<i32>) {
    match r {
        Ok(n) => println!("The first dobule is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn ex02() {
    let nums = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print2(double_first2(nums));
    print2(double_first2(empty));
    print2(double_first2(strings));
}

type Result3<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec3;

impl fmt::Display for EmptyVec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec3 {}

fn double_first3(v: Vec<&str>) -> Result3<i32> {
    v.first()
        .ok_or_else(|| EmptyVec3.into())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|n| n * 2))
}

fn print3(r: Result3<i32>) {
    match r {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn ex03() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print3(double_first3(numbers));
    print3(double_first3(empty));
    print3(double_first3(strings));
}

type Result4<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec4;

impl fmt::Display for EmptyVec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec4 {}

fn double_first4(v: Vec<&str>) -> Result4<i32> {
    let f = v.first().ok_or(EmptyVec4)?;
    let n = f.parse::<i32>()?;
    Ok(n * 2)
}

fn print4(r: Result4<i32>) {
    match r {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn ex04() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print4(double_first4(numbers));
    print4(double_first4(empty));
    print4(double_first4(strings));
}

type Result5<T> = std::result::Result<T, DoubleError5>;

#[derive(Debug)]
enum DoubleError5 {
    EmptyVec,
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError5::EmptyVec => write!(f, "please use a vector with at least one element"),
            DoubleError5::Parse(..) => write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError5 {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError5::EmptyVec => None,
            DoubleError5::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for DoubleError5 {
    fn from(err: ParseIntError) -> DoubleError5 {
        DoubleError5::Parse(err)
    }
}

fn double_first5(v: Vec<&str>) -> Result5<i32> {
    let f = v.first().ok_or(DoubleError5::EmptyVec)?;
    let parsed = f.parse::<i32>()?;
    Ok(parsed * 2)
}

fn print5(r: Result5<i32>) {
    match r {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("   Caused by: {}", source);
            }
        }
    }
}

fn ex05() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print5(double_first5(numbers));
    print5(double_first5(empty));
    print5(double_first5(strings));
}

/// Multiple error types
pub fn main() {
    // ex00();
    ex01();
    ex01_1();
    ex02();
    ex03();
    ex04();
    ex05();
}
