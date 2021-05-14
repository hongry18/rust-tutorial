macro_rules! say_hello {
    () => {
        println!("hello!");
    };
}

pub fn main() {
    say_hello!();
}
