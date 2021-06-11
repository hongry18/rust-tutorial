mod front_of_house;
mod front_of_house_dir {
    pub mod hosting_dir;
}

pub use crate::front_of_house::hosting;

pub use crate::front_of_house_dir::hosting_dir;

#[test]
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    hosting_dir::add_to_waitlist();
    front_of_house_dir::hosting_dir::add_to_waitlist();
}
