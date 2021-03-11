#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1,101);

    loop {
        print!("Enter your guess: ");
        stdout().flush().unwrap();

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range, try a number from 1 to 100");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e)
                    }
                }
            },
            Err(_) => continue,
        }
    }
}
