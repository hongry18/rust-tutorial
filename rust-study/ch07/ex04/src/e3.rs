mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("# ex3: add_to_waitlist")
        }
    }
}

use crate::e3::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

pub fn main() {
    eat_at_restaurant();
}
