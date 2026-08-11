use std::io::{self, Write};
mod shell;
mod shell_commands;
use shell::{Dispatch, Output};
use shell_commands::Command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Error while reading user input");

        let user_input = user_input.trim().to_string();
        let splited_text = user_input.split(" ").collect::<Vec<&str>>();
        let first_argument = splited_text[0].to_owned();
        let rest_of_arguments: Vec<String> = splited_text[1..]
            .to_vec()
            .into_iter()
            .map(|arg| arg.to_string())
            .collect();

        let cmd = Command::new(first_argument, rest_of_arguments);
        let dispatcher = Dispatch::new(cmd);
        match dispatcher.to_output() {
            Output::Text(msg) => {
                println!("{msg}");
            }
            Output::Exit => break,
        }
    }
}
