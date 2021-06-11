pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

pub fn reverse1(input: &str) -> String {
    let mut s = String::new();

    for c in input.chars().rev() {
        println!("c is {}", c);
        s.push(c);
    }

    s
}

#[test]
/// empty string
fn test1() {
    println!("### {}", reverse("abc"));
}
