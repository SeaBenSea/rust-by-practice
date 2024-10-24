use rand::Rng;
use std::{cmp::Ordering, io};

const MIN: usize = 1;
const MAX: usize = 100;

pub fn guessing_game() {
    let mut guess = String::new();
    let mut attempts = 0;
    let number = rand::thread_rng().gen_range(MIN..=MAX);
    println!("Guess a number between {} and {}!", MIN, MAX);

    loop {
        println!("Please input your guess:");

        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Invalid input. Please enter a number between {} and {}.",
                    MIN, MAX
                );
                continue;
            }
        };

        attempts += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&number) {
            Ordering::Equal => {
                println!(
                    "Congratulations! You guessed the correct number in {} tries!",
                    attempts
                );
                break;
            }
            Ordering::Greater => println!("Too high! Try a smaller number."),
            Ordering::Less => println!("Too low! Try a bigger number."),
        }
    }
}
