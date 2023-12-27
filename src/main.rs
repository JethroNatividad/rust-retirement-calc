use std::io;
use std::io::Write;
use chrono::{Datelike, Utc};


// Verbs: Determine, prompt, display
// Nouns: Program, years until retirement, year you can retire, current age, retirement age.

// Inputs: Current age, Retirement age
// Process: Years until retirement
// Output: You have {years_till_retirement} left until you can retire. \n It's {current_year}, so you can retire in {retirement_year}

fn main() -> io::Result<()> {
    let mut current_age = String::new();
    let mut retirement_age = String::new();
    
    print!("What is your current age? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut current_age)?;

    print!("At what age would you like to retire? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut retirement_age)?;

    let current_year = Utc::now().year();

    let current_age_parsed: i32 = current_age.trim().parse::<i32>().unwrap();
    let retirement_age_parsed: i32 = retirement_age.trim().parse::<i32>().unwrap();

    let years_till_retirement = retirement_age_parsed - current_age_parsed;
    let retirement_year = current_year + years_till_retirement;

    println!("You have {} years left until you can retire.\nIt's {}, so you can retire in {}.", years_till_retirement, current_year, retirement_year);

    Ok(())
}
