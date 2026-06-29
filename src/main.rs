use std::io::{self, Write};
mod shell_commands;
use shell_commands::{Command, CommandError, Commands};

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
        let command = Commands::from(&cmd);

        let new_command = match command {
            Ok(cmd) => cmd,
            Err(CommandError::CommandConversion(unsupported_command)) => {
                if Commands::search_for_bin(&cmd.directive).is_some() {
                    // If the custom binary is found then we run it as an external process
                    let output = std::process::Command::new(&cmd.directive)
                        .args(&cmd.args)
                        .output()
                        .expect("Fail to execute program");
                    if output.status.success() {
                        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
                        print!("{stdout}");
                    } else {
                        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
                        print!("{stderr}");
                    }
                    continue;
                } else {
                    println!("{unsupported_command}: command not found");
                    continue;
                }
            }
        };

        match new_command {
            Commands::TypeCommand(cmd) => {
                // this is argument passed to type (i.e type cat)
                let required_command = Command {
                    directive: cmd.args[0].to_owned(),
                    args: vec![],
                };
                if Commands::is_builtin(&required_command) {
                    println!("{} is a shell builtin", required_command.directive)
                } else {
                    if let Some(v) = Commands::search_for_bin(&required_command.directive) {
                        print!("{v}");
                    }
                }
            }
            Commands::Echo(cmd) => {
                println!("{}", cmd.args.join(" "))
            }
            Commands::Exit => {
                break;
            }
        }
    }
}
