use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io::*;
mod front_of_house;

fn serve_order(){}

pub mod back_of_house{
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order(){

    }

    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit:String
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

}
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant(){
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    let order = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    let secret_number = rand::thread_rng().gen_range(1,101);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}