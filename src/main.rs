use std::io;
use std::cmp::Ordering;
use  rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess: ");

    let mut input: String = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You guessed: {input}");
        }
        Err(error) => println!("error: {error}"),
    }

    let guess: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match guess.cmp(&secret_number) {
        Ordering::Equal => {
            println!("You win!");
            break;
        },
        Ordering::Less => println!("Guess too small"),
        Ordering::Greater => println!("Guess too big"),
    }
    }
}
