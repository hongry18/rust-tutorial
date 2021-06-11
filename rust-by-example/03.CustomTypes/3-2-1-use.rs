#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("the rich have lot of money!"),
        Poor => println!("the poor have no money..."),
    }

    match work {
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
