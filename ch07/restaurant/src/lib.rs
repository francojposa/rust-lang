// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn take_payment() {}
//     }
// }

mod front_of_house;

pub use crate::front_of_house::hosting;

// May be handled by front or back of house
fn serve_order() {}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Designating an enum as public makes all its variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// This method is just a mess of demonstration stuff
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change our mind about what bread we'd like
    meal.toast = String::from("Sourdough");

    // Next line won't compile if we uncomment it
    // We cannot see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let app1 = back_of_house::Appetizer::Soup;
    let app2 = back_of_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
