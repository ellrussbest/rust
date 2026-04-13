use colored::*;
use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number!");
  println!("Please input your guess.");

  let secret_number = rand::rng().random_range(1..=100);
  println!("The secret number is: {}", secret_number);

  loop {
    // new: Associated Method (Static methods)
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line.");

    // Shadowing (Guess was shadowed)
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed {}", guess);

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
