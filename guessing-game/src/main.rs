use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("{secret_number}");

    loop {
        println!("Please input the guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("write a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                return;
            }
            Ordering::Greater => println!("Too big"),
        }

        println!("You guessed: {}", guess);
    }

    
}



// generate a random number 
// if even print even
// if odd print odd
