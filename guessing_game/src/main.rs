use rand::Rng;
use std::{cmp::Ordering, io as deez};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        let _apples = 5;

        deez::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("Too high"),
        }
    }
}
