use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // generate a number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // introduce player and get name
    println!("Hello and welcome the number guessing game! What is your name?!");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    let name = name.trim();

    println!("Welcome {}!", name);

    // ask to guess a number and loop it till they get it right
    loop {
        println!("Try to guess the number I'm thinking.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        println!("You guessed: {}", guess);

        // make variables comparable and compare with matcher
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low, {}!", name),
            Ordering::Greater => println!("Too High, {}!", name),
            Ordering::Equal => {
                println!("Well done, {}! You guessed it! Huzzah!", name);
                break;
            }
        }
    }
}
