fn main() {
    let str = "abcd!@dZCV#32";
    print_alphabet(str);
}

fn print_alphabet(s: &str) {
    let bytes = s.as_bytes();
    for &i in bytes.iter() {
        if (i < 65 || i > 90) && (i < 95 || i > 122) {
            continue;
        }
        print!("{}", i as char)
    }
    println!("end");
}
