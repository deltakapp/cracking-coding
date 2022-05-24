use std::io;
mod is_unique;

fn main() {
    println!("Choose a problem 1-9:");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    println!("Running solution to problem {}", choice);

    is_unique::main(choice);
}
