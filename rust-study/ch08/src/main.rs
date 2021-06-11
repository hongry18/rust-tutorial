struct Point<T> {
    x: T,
    y: T
}

fn main() {
    // let str = "abcd!@dZCV#32";
    // print_alphabet(str);

    // let c = vec![String::from("a"), String::from("b")];
    // let t: String = c[0];

    let c = vec![1,2,3];
    let t = c[1];

    let s1 = String::from("test");
    println!("{:?}", s1.bytes());

    let p = Point{x:5, y: 5};
    println!("p.x = {}", p.x);
}

fn pp() -> Result<File, io::Error> {
    File::open("test")
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
