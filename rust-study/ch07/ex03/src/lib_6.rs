mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let absOrder1 = crate::lib_6::back_of_house::Appetizer::Soup;
    let absOrder2 = crate::lib_6::back_of_house::Appetizer::Salad;
}
