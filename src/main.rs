use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let answer: u32 = thread_rng().gen_range(1..10000000);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You must guess a number!");
                continue;
            }
        };

        match guess.cmp(&answer) {
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
            Ordering::Greater => println!("Too high, try again!"),
            Ordering::Less => println!("Too low, try again!"),
        }
    }
}
