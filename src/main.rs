#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    let mut user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Error while reading user input")
        .trim()
        .to_string();

    println!("{user_input}: command not found");

    io::stdout().flush().unwrap();
}
