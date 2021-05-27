mod back_of_house {
    pub struct Breakfast {
        //pub toast: String,
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn get_toast(&self) -> String {
            self.toast
        }

        pub fn set_toast(&self, toast: String) {
            self.toast = toast
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like

    // meal.toast = String::from("Wheat");
    meal.set_toast(String::from("Wheat"));
    // println!("I'd like {} toast please", meal.toast);
    println!("I'd like {} toast please", meal.get_toast());

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
