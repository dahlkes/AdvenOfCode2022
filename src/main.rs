use std::io;
mod advent;
use crate::advent::*;

fn main() -> io::Result<()> {
    println!("This is the advent of code 2022!");
    println!("Which day do you want to open?");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let day: i32 = user_input
                        .trim()
                        .parse()
                        .expect("Input not an integer");
    
    match day {
        1 => open_day_1(),
        2 => open_day_2(),
        3..=24 => println!("This day is not ready yet."),
        _ => println!("You are out of advent."),
    };

    Ok(())
}
