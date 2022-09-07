use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_no = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_no}");

    loop {
        println!("Input:");
        let mut guess = String::new(); // new is an associated function on String, but also found on
                                       // other types

        io::stdin()
            .read_line(&mut guess) // appends and does not overwrite
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // _ is for a catch all
                println!("Please enter a number");
                continue;
            }
        };

        match guess.cmp(&secret_no) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too great"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }

    // println!("You guessed: {guess}");
}
