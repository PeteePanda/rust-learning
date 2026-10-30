use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

fn main() {
  println!("Guess the number !");

  let secret_number = rand::rng().random_range(1..=100);

  println!("Secret Number: {secret_number}");

  loop {
    println!("Please input your guess.");
    
    // variables are immutable by default
    // so `mut` is needed to specify that a variable is mutable

    // String::new -- `::` indicates that the following is an 
    // associated function of the String type
    let mut guess = String::new(); // new emtpy instance of the String type

    io::stdin()
      .read_line(&mut guess) // Method of std::io, stores into `guess` variable (requires pass by reference)
      .expect("Failed ot read line"); // Output and exits on error; will get warning if not present.
    
    // Convert from String type to unsigned 32bit
    // trim() -- removes white spaces from beginning and end
    // parse() -- searches for the given type
    // let guess: u32 = guess.trim().parse().expect("Please type a number.");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Enter a Number.");
        continue;
      }
    };

    println!("You guessed: {guess}"); // does python formatting by default <3

    match guess.cmp(&secret_number) {
      // 3 possabilities when comparing numbers
      // <variant> => <execution>
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too large!"),
      Ordering::Equal => {
        println!("Winner !");
        break;
      }
    }
  }
}
