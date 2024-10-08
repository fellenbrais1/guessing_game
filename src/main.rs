use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

enum Difficulty {
    Easy = 8,
    Medium = 5,
    Hard = 3,
}

fn main() {

    // CONFIG
    // LOW cannot be equal to 0 or lower
    const LOW: u32 = 1;
    const HIGH: u32 = 100;
    
    let mut attempts;
    let secret_answer = rand::thread_rng().gen_range(LOW..=HIGH);

    println!("");
    println!("Welcome to the guessing game!");
    println!("");
    println!("What is your name, player?");
    println!("");

    let mut name: String = String::new();

    name = name.trim().to_string();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim().replace("\n", "");

    println!("");
    println!("I hope your guessing skills are strong {}!", name.purple());

    loop {
        println!("");
        println!("What level would you like to play?");
        println!("{}", "0. Quit".blue());
        println!("{}", "1. Easy".green());
        println!("{}", "2. Medium".yellow());
        println!("{}", "3. Hard".red());
        println!("");
    
        let mut choice: String = String::new();
    
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
    
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("");
                println!("Please enter one of the valid choices to continue!");
                continue;
            }
        };

        match choice {
            0 => {
                println!("");
                println!("Quitting the game!");
                return;
            }
            1 => {
                let difficulty = Difficulty::Easy;
                attempts = difficulty as u32;
                break;
            },
            2 => {
                let difficulty = Difficulty::Medium;
                attempts = difficulty as u32;
                break;
            },
            3 => {
                let difficulty = Difficulty::Hard;
                attempts = difficulty as u32;
                break;
            },
            _ => {
                println!("");
                println!("Please enter one of the valid choices to continue.");
                continue;
            }
        }
    }
    
    // DEBUG
    // println!("");
    // println!("The secret number is {}!", secret_answer);

    println!("");
    println!("I am thinking of a number between {} and {}, can you guess it?", LOW, HIGH);
    println!("You can enter '0' as a guess at any time to exit the game!");

    loop {
        if attempts == 0 {
            println!("You have run out of attempts, too bad {}!", name);
            println!("");
            println!("GAME OVER!");
            break;
        }
        
        if attempts == 1 {
            println!("You have 1 attempt remaining!");
        } else {
            println!("You have {} attempts remaining!", &attempts);
        }

        println!("");
        println!("Try and guess the number!");
        println!("");
        
        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        println!("");
        println!("You guessed: {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        if guess == 0 {
            println!("");
            println!("Exiting game!");
            break;
        }

        if guess > HIGH || guess < LOW {
            println!("Please choose a number between {} and {}!", LOW, HIGH);
            continue;
        }

        match guess.cmp(&secret_answer) {
            Ordering::Less => {
                println!("{}", "You guessed too small!".red());
                attempts -= 1;
            }
            Ordering::Equal => {
                println!("{}", "That's spot on!".green());
                println!(""); 
                println!("You win {}!", name);
                break;
            }
            Ordering::Greater => {
                println!("{}", "You guessed too large!".red());
                attempts -= 1;
            }
        }
    }
}
