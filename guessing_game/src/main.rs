use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You found the number!");
                break;
            }
            Ordering::Less => println!("It was too small!"),
            Ordering::Greater => println!("It was too large!"),
        }
        println!("You guessed: {}", guess);

    }
}
