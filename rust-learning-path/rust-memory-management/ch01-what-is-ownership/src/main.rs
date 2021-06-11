fn process(input: String) {}

fn process2(input: u32) {}

fn main() {
    {
        let mascot = String::from("ferris");
        let ferris = mascot;
        //println!("{}", mascot);
    }
    //println!("{}", mascot);

    let s = String::from("Hello, world!");
    process(s);
    //process(s);

    let s2 = "test";
    process(s2.to_string());

    let s = String::from("Hello, world!");
    process(s.clone());
    process(s);

    let n = 1u32;
    process2(n);
    process2(n);
}
