use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // range: start..=end
    let secret_number = rand::thread_rng().gen_range(0..=100).to_string();
    let secret_number: u32 = secret_number
        .trim()
        .parse()
        .expect("Secret number is not a number!");

    let mut counter = 1;

    loop {
        println!("");
        println!("[{counter}] Guess Number: ");

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(r) => r,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input must be a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("{guess} is LESS than the secret"),
            Ordering::Greater => print!("{guess} is GREATER than the secret"),
            Ordering::Equal => {
                print!("You win at your {counter}. guess!");
                break;
            }
        }

        counter += 1;
    }
}
