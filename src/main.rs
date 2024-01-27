use rand::Rng;
use std::io;
use std::io::Write;
use colored::*;

fn get_random_char(length: usize, letters: &str) -> char {
    let rand_index = rand::thread_rng().gen_range(0..length);
    letters.chars().nth(rand_index).unwrap()
}

fn main() {
    let letters: &str = "abcdefghijklmnopqrstuvwzyz";
    let upper_case = letters.to_uppercase();
    let upper_case = upper_case.as_str();
    let digits = "0123456789";
    let punctuations = "`~!@#$%^&*()-_=+,<>./?|\\:;'\"[]{}";
    println!("{}", "\n############# Random Password Generator ###############\n".green());
    println!("{}", "0 - Only Lowercase\n\
    1 - Only Uppercase\n\
    2 - Both Uppercase and Lowercase\n\
    3 - Only Digits\n\
    4 - Only Punctuations\n\
    5 - All
    ");
    print!("Select any one option > ");
    io::stdout()
        .flush()
        .expect("Unable to process the request");

    let mut user_input = String::new();
    let mut length = String::new();
    io::stdin()
        .read_line(&mut user_input).expect("paniced");
    print!("Enter your password length: > ");
    io::stdout()
        .flush()
        .expect("panicked");

    io::stdin()
        .read_line(&mut length)
        .expect("panicked");

    let user_input: i32 = user_input
        .trim()
        .parse().unwrap_or_else(|_| 5);
    let length: usize = length.trim().parse().unwrap_or_else(|_| 6);
    let mut password = String::new();
    for _ in 0..length {
        match user_input {
            0 => {
                // Only Lowercase
                let  random_char = get_random_char(letters.len(), letters);
                password.push(
                    random_char
                );
            },
            1 => {
                // Only Uppercase
                let random_char = get_random_char(upper_case.len(), &upper_case);
                password.push(
                    random_char
                );
            },
            2 => {
                // Both
                let random_char = get_random_char(letters.len() + upper_case.len(), &format!("{}{}", letters, upper_case));
                password.push(
                    random_char
                );
            },
            3 => {
                // Only Digits
                let random_char = get_random_char(digits.len(), digits);
                password.push(
                    random_char
                );
            },
            4 => {
                // Only Punctuations
                let random_char = get_random_char(punctuations.len(), punctuations);
                password.push(
                    random_char
                );
            },
            5 => {
                // All
                let random_source = format!("{}{}{}{}", letters, upper_case, digits, punctuations);
                let random_char = get_random_char(random_source.len(), &random_source);
                password.push(random_char);
            }
            _ => {
                println!("Something went wrong!")
            }
        }
    }



    println!("Your password is: {}", password.green())
}