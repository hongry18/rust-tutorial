fn main() {
    println!("Hello, world!");

    let s1 = String::from("s1");
    let s2 = String::from("s2");

    println!("s3 is {}", sum_str_plus_operator(&s1, &s2));
    println!("s3 is {}", sum_str_plus_operator(&s1, &s2));

}

fn sum_str_plus_operator(x: &String, y: &String) -> String {
    return "sum: ".to_string() + x + y
}