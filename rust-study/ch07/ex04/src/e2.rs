mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("# ex2: add 2 waitlist");
        }
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn main() {
    eat_at_restaurant();
}
