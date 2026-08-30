use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Define the range for the guessing game
const MIN_RANGE: u32 = 1;
const MAX_RANGE: u32 = 100;

fn main() {
    println!("Pick a Number so I can guess");

    // Generate a random number between 1 and 100 (inclusive) but now using MAX and MIN const to ensure exact
    let secret_number = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);

    // For debugging purposes: reveal the secret number
    // println!("The secret number is:{secret_number}");

    loop {
        println!("Please input a number:");

        let mut guess = String::new();

        // Read input from the standard input stream
        guess.clear(); // Clear the old input but keep the memory buffer
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Parse the input string into a 32-bit unsigned integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // If parsing fails (e.g., non-numeric input), skip the rest of the loop
            Err(_) => {
                print!("Please enter a valid number :)");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Comparison of user's guess and the secret number to determine the next action.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!!! :)");
                // Break out of the loop since the user has guessed correctly
                break;
            }
        }
    }
}
