use rand::Rng;
use std::io;
use std::io::Write;
use colored::*;

fn main() {
    let  random: i32 = rand::thread_rng().gen_range(1..10);
    let mut tries = 0;
    while tries < 3 {
        let mut input = String::new();
        print!("Enter input: ");
        io::stdout().flush().expect("Something went wrong");
        io::stdin().read_line(&mut input).expect("something went wrong");
        let input: i32 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => panic!("something went wrong")
        };
        match input {
            -1 => println!("Invalid input"),

            _ if input == random => {
                println!("You win");
                break;
            },
            _ if tries != 2 => {
                println!("Try again!\n");
                let tries_left = 2 - tries;
                let formatted_string = "You have only {tries_left} attempts\n".yellow();
                println!("{}", formatted_string)
            }
            _ => println!("{} {}", "You loose, the secret no is".red(), random=random)
        }
        tries +=1;
    }
}