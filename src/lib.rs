pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    pub fn cook_order() {
        // super refers to parent module.
        super::eat_at_restaurant();
    }

    // Making struct public
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        // if you don't add to the field then you cannot access it directly.
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    front_of_house::cook_order();

    let mut summer_breakfast = front_of_house::Breakfast::summer("toasty");
    summer_breakfast.toast = String::from("Wheat");
    println!("My breakfast: {:?}", summer_breakfast);
}
