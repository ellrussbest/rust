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

  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");

  // cannot create breakfast struct directly beacuse it has one private field
  // illegal
  //   const bf = back_of_house::Breakfast {
  //     toast: String::from("Wheat"),
  //     seasonal_fruit: String::from("peaches")
  //   };

  // Enum options are automitacally public when the enum is public
  let appetizer: back_of_house::Appetizer = back_of_house::Appetizer::Salad;
}
