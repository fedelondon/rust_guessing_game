use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, stdout, BufWriter};
use ferris_says::say;

fn main() {

    guessing_ferris();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faild to read.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guesses {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

fn guessing_ferris(){
   let stdout = stdout();
   let message = String::from("Hello to Guess the number Ferris");
   let width = message.chars().count();

   let mut writer = BufWriter::new(stdout.lock());
   say(message.as_bytes(), width, &mut writer).unwrap();
}
