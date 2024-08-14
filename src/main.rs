use rand::Rng;
use std::io;

fn main() {
    println!("Pick a Numbers so I can guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is :{secret_number}");

    println!("Please input a number:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    println!("You guess correctly");
}
