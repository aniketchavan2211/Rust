use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    #![allow(warnings)]

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    let max_attempts = 3;
    let mut incorrect_guesses = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        incorrect_guesses += 1;

        let chances_left = max_attempts - incorrect_guesses;
        println!("Chances left: {}", chances_left);

        if incorrect_guesses == max_attempts {
            println!("Sorry, you've reached the maximum number of incorrect guesses. The correct number was: {}", secret_number);
            break;
        }
    }
}
