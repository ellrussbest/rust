// Child modules and their items are private unless marked `pub`.
// A child module can access items defined by its parent module.

pub mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {
      println!("Add to waitlist");
    }
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn server_order() {}
    fn take_payment() {}
  }
}

pub fn eat_at_restaurant() {
  // absolute path
  crate::module::example::front_of_house::hosting::add_to_waitlist();

  // relative path
  front_of_house::hosting::add_to_waitlist();
}
