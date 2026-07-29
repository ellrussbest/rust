use colored::*;
use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number!");
  println!("Enter a number between 1 and 100.");

  let secret_number = rand::rng().random_range(1..=100);

  loop {
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line.");

    // Shadow the input string with its parsed numeric value.
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("{}", "Too small!".red()),
      Ordering::Greater => println!("{}", "Too big!".red()),
      Ordering::Equal => {
        println!("{}", "You win!".green());
        break;
      }
    }
  }
}
