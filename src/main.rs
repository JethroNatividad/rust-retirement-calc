use std::io;
use std::io::Write;
use chrono::{Datelike, Utc};


// Verbs: Determine, prompt, display
// Nouns: Program, years until retirement, year you can retire, current age, retirement age.

// Inputs: Current age, Retirement age
// Process: Years until retirement
// Output: You have {years_till_retirement} left until you can retire. \n It's {current_year}, so you can retire in {retirement_year}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    let current_age: i32 = get_input("What is your current age? ");
    let retirement_age: i32 = get_input("At what age would you like to retire? ");

    let current_year = Utc::now().year();

    let years_till_retirement = retirement_age - current_age;
    let retirement_year = current_year + years_till_retirement;

    println!("You have {} years left until you can retire.\nIt's {}, so you can retire in {}.", years_till_retirement, current_year, retirement_year);
}
