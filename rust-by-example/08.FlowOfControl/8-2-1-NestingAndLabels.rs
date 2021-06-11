#[allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer
        }

        println!("This point will never be reached");
    }

    println!("Existed the outer loop");
}
