// Implements solutions to chapter 1 problems

use std::io;
mod is_unique;

// Selects and runs solutions
fn main() {
    let solution = is_unique::main(get_input_string());
    println!("{}", solution);
}

// get a string from user input
fn get_input_string() -> String {
    println!("enter a string");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}
