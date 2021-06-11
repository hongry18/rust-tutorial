fn print_greeting(msg: &String) {
    println!("Greeting: {}", msg);
}

/*
// panic code
fn change(msg: &String) {
    msg.push_str("1");
}
*/

fn change(msg: &mut String) {
    msg.push_str(", !");
}

fn main() {
    let greeting = String::from("Hello");
    let greeting_reference = &greeting;
    println!("Greeting: {}", greeting);
    println!("Greeting Ref: {}", greeting_reference);

    print_greeting(&greeting);
    print_greeting(&greeting_reference);

    let mut greeting = String::from("mut greeting");
    print_greeting(&greeting);
    change(&mut greeting);
    print_greeting(&greeting);

    let mut value = String::from("hello");

    /*
    let ref1 = &mut value;
    let ref2 = &mut value;

    println!("{}, {}", ref1, ref2);
    */
}
