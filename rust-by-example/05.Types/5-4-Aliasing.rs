type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nano_seconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoSeconds + {} inches = {} unit?",
            nano_seconds,
            inches,
            nano_seconds + inches);
}
