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
        let splited_text: Vec<&str> = user_input.split(" ").map(|arg| arg).collect();
        let first_argument = splited_text[0];
        let rest_of_arguments = splited_text[1..].join(" ");

        if first_argument == "exit" {
            break;
        } else if first_argument == "echo" {
            println!("{rest_of_arguments}");
        } else {
            println!("{user_input} command not found");
        }
    }
}
