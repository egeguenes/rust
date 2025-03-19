use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!");

    let answer = rand::thread_rng().gen_range(1, 101);
    println!("The answer is {}", answer);

    loop {
        println!("Input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&answer) {
            Ordering::Less => println!("{} than that!", "Bigger".red()),
            Ordering::Greater => println!("{} than that!", "Lesser".red()),
            Ordering::Equal => {
                println!("{}", "Exactly the right answer! You win!!!".green());
                break;
            }
        };
    }
}
