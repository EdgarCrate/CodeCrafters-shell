use crate::shell_commands::{AvailableCommands, Command};
use std::env;
use std::path::PathBuf;
pub enum Output {
    Text(String),
    Exit,
}

pub enum Dispatch {
    Builtin(AvailableCommands, Command),
    External(Command),
    NotFound(String),
}

impl Dispatch {
    pub fn new(cmd: Command) -> Dispatch {
        match AvailableCommands::from(&cmd.directive) {
            Ok(bultin) => Dispatch::Builtin(bultin, cmd),
            Err(_) => match AvailableCommands::search_for_bin(&cmd.directive) {
                Some(_) => Dispatch::External(cmd),
                None => Dispatch::NotFound(cmd.directive),
            },
        }
    }

    pub fn to_output(&self) -> Output {
        match self {
            Dispatch::Builtin(shell_commands, user_command) => match shell_commands {
                AvailableCommands::TypeCommand => {
                    let bin_args = user_command.args.get(0).map(|s| s.as_str()).unwrap_or("");
                    if AvailableCommands::is_builtin(bin_args) {
                        return Output::Text(format!("{} is a shell builtin", bin_args));
                    } else {
                        if let Some(path) = AvailableCommands::search_for_bin(&bin_args) {
                            return Output::Text(format!("{} is {}", bin_args, path.display()));
                        } else {
                            return Output::Text(format!("{}: not found", bin_args));
                        }
                    }
                }
                AvailableCommands::Exit => Output::Exit,
                AvailableCommands::Echo => Output::Text(user_command.args.join(" ")),
            },
            Dispatch::External(cmd) => {
                let cwd = env::current_dir().expect("Could not determine current directory");
                let message_output =
                    AvailableCommands::run_external_process(&cmd, &PathBuf::from(cwd));
                Output::Text(message_output)
            }
            Dispatch::NotFound(msg) => Output::Text(format!("{}: command not found", msg)),
        }
    }
}
