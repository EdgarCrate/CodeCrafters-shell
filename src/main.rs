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
        let splited_text = user_input.split(" ").map(|arg| arg).collect::<Vec<&str>>();
        let first_argument = splited_text[0];
        let rest_of_arguments = splited_text[1..].join(" ");
        match first_argument {
            "exit" => break,
            "echo" => println!("{rest_of_arguments}"),
            _ => println!("{user_input} command not found"),
        }
    }
}
