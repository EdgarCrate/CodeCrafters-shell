use crate::shell_commands::{AvailableCommands, Command};
use std::path::PathBuf;

pub enum Output {
    Text(String),
    Exit,
}

pub enum Dispatch {
    Builtin(AvailableCommands),
    External(Command, PathBuf),
    NotFound(String),
}

impl Dispatch {
    pub fn new(cmd: Command) -> Dispatch {
        match AvailableCommands::from(&cmd) {
            Ok(bultin) => Dispatch::Builtin(bultin),
            Err(_) => match AvailableCommands::search_for_bin(&cmd.directive) {
                Some(path) => Dispatch::External(cmd, PathBuf::from(path)),
                None => Dispatch::NotFound(cmd.directive),
            },
        }
    }

    pub fn to_output(&self) -> Output {
        match self {
            Dispatch::Builtin(commands) => match commands {
                AvailableCommands::TypeCommand(cmd) => {
                    let bin_name = &cmd.args[0];
                    if AvailableCommands::is_builtin(&cmd) {
                        return Output::Text(format!("{} is a shell builtin", bin_name));
                    } else {
                        if let Some(path) = AvailableCommands::search_for_bin(&bin_name) {
                            return Output::Text(format!("{} is {}", bin_name, path.display()));
                        } else {
                            return Output::Text(format!("{}: command not found", bin_name));
                        }
                    }
                }
                AvailableCommands::Exit => Output::Exit,
                AvailableCommands::Echo(args) => Output::Text(args.join(" ").trim().to_owned()),
            },
            Dispatch::External(cmd, path) => {
                let message_output = AvailableCommands::run_external_process(&cmd, path);
                Output::Text(message_output)
            }
            Dispatch::NotFound(msg) => Output::Text(format!("{}: command not found", msg)),
        }
    }
}
