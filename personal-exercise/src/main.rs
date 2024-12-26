mod temperature_converter;
mod fibonacci;

use std::io;
use temperature_converter::convert;
use fibonacci::predict;

fn main() {
    println!("Hello, user.\nWhat would like to do today?");

    
    let mut input = String::new();
    let mut choice: u8;

    loop {
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        choice = input.trim().parse().expect("Error");

        match choice {
            1 => convert(),
            2 => predict(),
            _ =>  {
                println!("Please enter a valid choice from the provided choices 1");
            
            },
        }
    }

    
}
