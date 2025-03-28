use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::time;

fn main() {
    println!("Guessing Game!");

    let answer = rand::thread_rng().gen_range(1, 1000001);
    println!("SECRET: The answer is {}", answer);

    println!("Please select the game mode: 0 default / 1 bot");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read");
    let mode: u32 = match mode.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if mode == 0 {
        game_person(answer);
    } else if mode == 1 {
        game_computer(answer);
    } else {
        return;
    }

    return;
}

fn game_computer(answer: u32) {
    let mut max_val = 1000001;
    let mut min_val = 1;
    let mut current_guess = rand::thread_rng().gen_range(min_val, max_val);
    loop {
        let ten_millis = time::Duration::from_millis(100);
        std::thread::sleep(ten_millis);
        println!("Guess is {}", current_guess);

        match current_guess.cmp(&answer) {
            Ordering::Less => {
                println!("{} than that!", "Bigger".to_uppercase().red());
                min_val = current_guess + 1;
                current_guess = rand::thread_rng().gen_range(min_val, max_val);
            }
            Ordering::Greater => {
                println!("{} than that!", "Lesser".red());
                max_val = current_guess - 1;
                current_guess = rand::thread_rng().gen_range(min_val, max_val);
            }
            Ordering::Equal => {
                println!("{}", "Exactly the right answer! You win!!!".green());
                break;
            }
        };
    }
}

fn game_person(answer: u32) {
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
            Ordering::Less => println!("{} than that!", "Bigger".to_uppercase().red()),
            Ordering::Greater => println!("{} than that!", "Lesser".red()),
            Ordering::Equal => {
                println!("{}", "Exactly the right answer! You win!!!".green());
                break;
            }
        };
    }
}
