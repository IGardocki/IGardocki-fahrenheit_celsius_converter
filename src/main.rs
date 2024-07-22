use std::io;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}

fn main() {
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse()
        .expect("Please enter a number");

    match choice {
        1 => {
            println!("Enter temperature in Celsius:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");
            let celsius: f64 = input.trim().parse()
                .expect("Please enter a number");

            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("{}°C is equal to {}°F", celsius, fahrenheit);
        },
        2 => {
            println!("Enter temperature in Fahrenheit:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");
            let fahrenheit: f64 = input.trim().parse()
                .expect("Please enter a number");

            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{}°F is equal to {}°C", fahrenheit, celsius);
        },
        _ => println!("Invalid choice. Please enter 1 or 2.")
    }
}
