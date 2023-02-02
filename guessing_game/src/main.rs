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

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = guess.trim().parse().expect("Plese type a number");

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
