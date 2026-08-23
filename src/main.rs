use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Pick a Number so I can guess");

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // For debugging purposes: reveal the secret number
    // println!("The secret number is:{secret_number}");

    loop {
        println!("Please input a number:");

        let mut guess = String::new();

        // Read input from the standard input stream
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Parse the input string into a 32-bit unsigned integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // If parsing fails (e.g., non-numeric input), skip the rest of the loop
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare the user's guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!!! :)");
                break;
            }
        }
    }
}
