#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Error while reading user input");

        let user_input = user_input.trim().to_string();

        if user_input == "exit" {
            break;
        }

        println!("{user_input}: command not found");
    }
}
