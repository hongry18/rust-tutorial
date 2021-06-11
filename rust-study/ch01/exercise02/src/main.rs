use std::io;

fn main() {
    println!("Please input words.");

    let mut words = String::new();

    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read line");

    let bytes = words.as_bytes();
    let loop_end = bytes.len();
    // for loop 역순을 어떻게 하는지 몰라서 헤맴
    for n in (0..loop_end).rev() {
        print!("{}", bytes[n] as char);
    }
}
