use std::io;

fn main() {
    println!("Hello, world!");

    let mut input: String = String::new();

    println!("First name: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to grab the input");

    let first_name: String = input.trim().to_string();
    input.clear();

    println!("Last name: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to grab the input");

    let last_name = input.trim().to_string();
    input.clear();

    println!("Age: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to grab the input");

    let age: u16 = input.trim().parse().expect("error");

    let user = Person::init(&first_name, &last_name, age);

    println!("{:?}", user)
    

}

// todo: make it modular and reduce io input code 
// how to get enum from user input

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u16,
    gender: Gender,
}

impl Person {

    fn  init (first_name: &String, last_name: &String, age: u16) -> Self {
        Self{
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            gender: Gender::Male
        }
    }
}

#[derive(Debug)]
enum Gender{
    Male,
    Female
}