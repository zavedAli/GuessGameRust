use std::io;
use rand::Rng;



fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret Number: {} ",secret_number);

    loop {
        println!("Please input your guess (or 'b' to quit):");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Trim the input to remove any extra whitespace and convert it to lowercase.
        let guess = guess.trim().to_lowercase();

        // Check if the user wants to quit with 'b'.
        if guess == "b" {
            println!("Quitting game.");
            break;
        }

        // Try to parse the guess as a number.
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number or 'b' to quit.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Using if conditions to check the guess against the secret number.
        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }
}
