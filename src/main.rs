use std::{io, num::ParseIntError, cmp::Ordering};

fn main() {
    println!("Guess the number");
    println!("Please input your guess: ");

    let mut input: String = String:: new();
    let secret_number: u32 = 40;
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("You guessed: {input}");
        }
        Err(error) => println!("error: {error}"),
    }

    let guess: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Less => println!("Guess too small"),
        Ordering::Greater => println!("Guess too big"),
    }
}
