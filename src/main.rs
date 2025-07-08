use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!\n");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line\n");

        let guess = guess.trim();

        if guess.eq_ignore_ascii_case("quit") {
            println!("\nGoodbye!");
            break;
        }


        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number or type 'quit' to exit.\n");
                continue;
            }
        };

        println!("You guessed: {guess}\n");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win! The secret number was {secret_number}.");
                break;
            }
        }

    }
}
