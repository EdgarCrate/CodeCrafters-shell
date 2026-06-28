use std::ffi::OsString;
use std::fs::DirEntry;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs};

#[derive(PartialEq, Debug)]
enum Commands {
    TypeCommand(Command),
    Exit,
    Echo(Command),
}

enum CommandError {
    CommandConversion(String),
}

#[derive(PartialEq, Debug, Clone)]
struct Command {
    directive: String,
    args: Vec<String>,
}

impl Command {
    fn new(directive: String, args: Vec<String>) -> Self {
        Command { directive, args }
    }
}

// Please separate responsabilities
// Create another function that tells should that the command is valid
// If it is valid create instance of the variant and the command.

impl Commands {
    fn from(cmd: Command) -> Result<Commands, CommandError> {
        match cmd.directive.as_str() {
            "type" => Ok(Commands::TypeCommand(cmd)),
            "exit" => Ok(Commands::Exit),
            "echo" => Ok(Commands::Echo(cmd)),
            _ => Err(CommandError::CommandConversion(cmd.directive.clone())),
        }
    }
    fn is_built(cmd: &Command) -> bool {
        Commands::from(cmd.clone()).is_ok()
    }
    fn read_path() -> OsString {
        env::var_os("PATH").expect("PATH variable is not available")
    }
    fn is_file_executable(path: &Path) -> bool {
        let metadata = fs::metadata(&path).expect("Error while reading metadata");
        let mode = metadata.permissions().mode();
        (mode & 0o111) != 0
    }
    fn search_for_bin(cmd: &Command) {
        let path_value = Commands::read_path();
        let list_of_directories = env::split_paths(&path_value);
        // Given then when
        // Given a list of paths,
        // Then check if the dir can be opened
        // and check if one of the items in that dir matches the directive name
        // and then check if the file has execute permissions
        // if finded return
        // if not continue to the next directory to be openend
        let bin = &cmd.args[0];
        for dir in list_of_directories {
            match fs::read_dir(&dir) {
                Ok(entries) => {
                    let dir_items: Vec<DirEntry> = entries.filter_map(|e| e.ok()).collect();
                    let directive_item = dir_items
                        .into_iter()
                        .filter_map(|item| {
                            let name = item.file_name().into_string().ok()?;
                            Some((item, name))
                        })
                        .find(|(_, name)| name == bin)
                        .map(|(item, _)| item);
                    if let Some(executable) = directive_item {
                        if Commands::is_file_executable(&executable.path()) {
                            println!("{} is {}", bin, executable.path().display());
                            return;
                        }
                    } else {
                        continue;
                    }
                }
                Err(e) => eprintln!("Could not read the directory {}", e),
            }
        }

        println!("{}: not found", bin)
    }
}

// fn is_bin_in_path(path_value: OsString, bin_name: String) {
//     let list_of_paths = env::split_paths(&path_value);
//     for path in list_of_paths.into_iter() {
//         for dirs in fs::read_dir(&path).unwrap() {
//             let dir_item = dirs.unwrap();
//             let dir_item_name = dir_item.file_name().into_string().unwrap();
//             if dir_item_name == bin_name {
//                 let metadata = fs::metadata(dir_item.path()).expect("Error while reading metadata");
//                 let mode = metadata.permissions().mode();
//                 let is_executable = (mode & 0o111) != 0;
//                 if is_executable {
//                     println!("{bin_name} is {}", dir_item.path().to_str().unwrap());
//                     return;
//                 } else {
//                     continue;
//                 }
//             }
//         }
//     }
//     println!("{bin_name}: not found");
// }

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
        let command = match Commands::from(cmd) {
            Ok(cmd) => cmd,
            Err(CommandError::CommandConversion(unsupported_command)) => {
                println!("{unsupported_command}: command not found");
                continue;
            }
        };
        match command {
            Commands::TypeCommand(cmd) => {
                let required_command = Command {
                    directive: cmd.args[0].to_owned(),
                    args: vec![],
                };
                if Commands::is_built(&required_command) {
                    println!("{} is a shell builtin", cmd.directive)
                } else {
                    Commands::search_for_bin(&cmd);
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
