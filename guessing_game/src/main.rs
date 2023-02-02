use rand::Rng;
use std::{cmp::Ordering, io};

fn random_number(start: u32, end: u32) -> i32 {
    let secret_number: i32 = match rand::thread_rng()
        .gen_range(start..=end)
        .to_string()
        .trim()
        .parse()
    {
        Ok(n) => n,
        Err(_) => {
            println!("Something went wrong generating random number. Returning 1.");
            -1
        }
    };

    secret_number
}

fn read_input(counter: i32) -> i32 {
    let mut guess = String::new();

    println!("[{counter}] Guess Number: ");
    io::stdin().read_line(&mut guess).unwrap();

    let guess = match guess.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Input must be a number");
            -1
        }
    };
    guess
}

fn compare_numbers(input_number: i32, secret_number: i32) -> i32 {
    let mut result = -1;

    match input_number.cmp(&secret_number) {
        Ordering::Less => println!("Input LESS than secret"),
        Ordering::Greater => println!("Input MORE than secret"),
        Ordering::Equal => {
            println!("You win!");
            result = 1;
        }
    };

    result
}

fn main() {
    let mut counter = 1;

    let secret_number = random_number(0, 100);

    println!("Secret: {secret_number}");
    loop {
        let input_number = read_input(counter);
        if input_number < 0 || input_number > 100 {
            println!("Input must be between 0 and 100 inclusive.");
            continue;
        }

        let result = compare_numbers(input_number, secret_number);

        if result == 1 {
            break;
        }

        counter += 1;
    }
}
