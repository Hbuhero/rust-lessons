use std::io;

fn to_celcius(fahrenheit: f64 ) -> f64 {
    return (fahrenheit - 32.0) * 5.0/9.0;
}

fn to_fahrenheit(celcius: f64) -> f64 {
    return (celcius * 9.0/5.0) + 32.0;
}

pub fn convert() {
    let mut celcius = String::new();

    io::stdin()
        .read_line(&mut celcius)
        .expect("Enter a proper temp value");

    let celcius: f64 = celcius
        .trim()
        .parse()
        .expect("Failed to convert to a number");

    println!("The Fahrenheit value is: {}", to_fahrenheit(celcius));
}
