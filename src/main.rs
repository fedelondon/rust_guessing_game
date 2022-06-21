use colored::*;
use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, stdout, BufWriter};

fn main() {
    guessing_ferris();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    let mut count_try = 1;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faild to read.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess <= 100 {
            println!("You guesses {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too small!".red()),
                Ordering::Greater => println!("{}", "Too big!".red()),
                Ordering::Equal => {
                    println!(
                        "{} {} {}",
                        "You win in".green().bold(),
                        count_try,
                        "try's".green().bold()
                    );
                    println!("{} {} !", "it".green(), "works".blue().bold());
                    break;
                }
            }
        } else {
            println!("The number you guess is bigger than 100");
        }
        count_try += 1;
    }
}

fn guessing_ferris() {
    let stdout = stdout();
    let message =
        String::from("Hello to Guess the number Ferris\nPlease guess a number between 1 to 100");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
