use std::io::{self, Write};
const DEFAULT_MSG: &str = "is a shell builtin";

#[derive(PartialEq, Debug)]
enum Commands {
    TypeCommand,
    Exit,
    Echo,
}

type Result<T> = std::result::Result<T, CommandError>;

enum CommandError {
    CommandConversion(String),
}

impl Commands {
    fn from(value: &str) -> Result<Commands> {
        match value {
            "type" => Ok(Commands::TypeCommand),
            "exit" => Ok(Commands::Exit),
            "echo" => Ok(Commands::Echo),
            _ => Err(CommandError::CommandConversion(value.to_string())),
        }
    }

    fn is_built(arg: &str) {
        match Commands::from(arg) {
            Ok(_) => println!("{arg} {DEFAULT_MSG}"),
            Err(_) => println!("{arg}: not found"),
        }
    }
}

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

        let command = match Commands::from(first_argument) {
            Ok(cmd) => cmd,
            Err(CommandError::CommandConversion(unsupported_command)) => {
                println!("{unsupported_command}: command not found");
                continue;
            }
        };

        match command {
            Commands::TypeCommand => Commands::is_built(&rest_of_arguments),
            Commands::Echo => {
                println!("{rest_of_arguments}")
            }
            Commands::Exit => {
                break;
            }
        }
    }
}
