use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let target = rand::thread_rng().gen_range(0, 100);
    println!("Target is {}", target.to_string().blue());
    println!("Hi, enter a number");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Not a valid number, Try again!".red());
                continue;
            }
        };
        match guess.cmp(&target) {
            Ordering::Less => println!("{}", "Chiisai".red()),
            Ordering::Greater => println!("{}", "Oo sugi".red()),
            Ordering::Equal => {
                println!("{}", "Katta!".green());
                break;
            }
        };
    }
}
