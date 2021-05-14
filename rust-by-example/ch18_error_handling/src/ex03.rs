use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();

    first_number + second_number
}

fn ex01() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("dobule is {}", tt);
}

fn ex02() -> Result<(), ParseIntError> {
    let number_str = "5";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    println!("{}", number);
    Ok(())
}

// early returns 이미 사용하고있었네
fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn ex03() {
    let twenty = multiply2("10", "2");
    print(twenty);

    let tt = multiply2("10", "a");
    print(tt);
}

fn multiply3(s1: &str, s2: &str) -> Result<i32, ParseIntError> {
    s1.parse::<i32>()
        .and_then(|n1| s2.parse::<i32>().map(|n2| n1 * n2))
}

fn ex04() {
    let t1 = multiply3("10", "2");
    print(t1);

    let t2 = multiply3("10", "bb");
    print(t2);
}

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply4(s1: &str, s2: &str) -> AliasedResult<i32> {
    s1.parse::<i32>()
        .and_then(|n1| s2.parse::<i32>().map(|n2| n1 * n2))
}

fn ex05() {
    let t1 = multiply4("10", "2");
    print(t1);

    let t2 = multiply4("10", "bb");
    print(t2);
}

fn multiply5(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn ex06() {
    let t1 = multiply5("10", "2");
    print(t1);

    let t2 = multiply5("10", "bb");
    print(t2);
}

fn multiply6(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let n1 = first_number_str.parse::<i32>()?;
    let n2 = second_number_str.parse::<i32>()?;

    Ok(n1 * n2)
}

fn ex07() {
    print(multiply6("10", "2"));
    print(multiply6("10", "t"));
}

// fn multiply7(s1: &str, s2: &str) -> Result<i32, ParseIntError> {
//     // try macro deprecated
//     let n1 = try!(s1.parse::<i32>());
//     let n2 = try!(s2.parse::<i32>());

//     Ok(n1 * n2)
// }

pub fn main() {
    // ex01();
    // ex02();
    ex03();
    ex04();
    ex05();
    ex06();
    ex07();
}
