mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("# ex1: add_to_waitlist")
        }
    }
}

use crate::e1::front_of_house::hosting;
// 외부에서도 hosting을 사용하려면
// pub use crate::e1::front_of_house::hosting; 이건 7.4

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn main() {
    eat_at_restaurant();
}

// re exporting names with pub use ??
// pub use crate::e1::front_of_house::hosting;
