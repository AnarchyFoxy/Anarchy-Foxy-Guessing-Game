extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
   println!("Hello, dear guest. Let's play! Guess the number!");

   let secret_number = rand::thread_rng().gen_range(1..101);

   println!("The secret number is: {}", secret_number);

   loop {
      println!("Please input your guess.");

      let mut guess_number = String::new();

      io::stdin().read_line(&mut guess_number).expect("Failed to read line");

      let guess_number: u32 = match guess_number.trim().parse() {
         Ok(num) => num,
         Err(_) => continue,
      };

      println!("You guessed: {}", guess_number);

      match guess_number.cmp(&secret_number) {
      Ordering::Less => println!("This is too small!"),
      Ordering::Greater => println!("This is too big!"),
      Ordering::Equal => {
         println!("You win! Congrats!");
         break;
         }
      }
   }
}
