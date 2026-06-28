use std::ffi::OsString;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::{env, fs};

#[derive(PartialEq, Debug)]
enum Commands {
    TypeCommand,
    Exit,
    Echo,
}

enum CommandError {
    CommandConversion(String),
}

impl Commands {
    fn from(value: &str) -> Result<Commands, CommandError> {
        match value {
            "type" => Ok(Commands::TypeCommand),
            "exit" => Ok(Commands::Exit),
            "echo" => Ok(Commands::Echo),
            _ => Err(CommandError::CommandConversion(value.to_string())),
        }
    }

    fn is_built(arg: &str) -> bool {
        Commands::from(arg).is_ok()
    }
}

fn is_bin_in_path(path_value: OsString, bin_name: String) {
    let list_of_paths = env::split_paths(&path_value);
    for path in list_of_paths.into_iter() {
        for dirs in fs::read_dir(&path).unwrap() {
            let dir_item = dirs.unwrap();
            let dir_item_name = dir_item.file_name().into_string().unwrap();
            if dir_item_name == bin_name {
                let metadata = fs::metadata(&path).expect("Error while reading metadata");
                let mode = metadata.permissions().mode();
                let is_executable = (mode & 0o111) != 0;
                if is_executable {
                    println!("{bin_name} is {}", path.to_path_buf().to_str().unwrap())
                } else {
                    continue;
                }
            }
        }
    }
    println!("{bin_name}: not found");

    // let found = list_of_paths
    //     .into_iter()
    //     .flat_map(|p| fs::read_dir(p).unwrap())
    //     .find(|item| {
    //         let dir_item = item.as_ref().unwrap();
    //         let dir_item_name = dir_item.file_name().into_string().unwrap();
    //         dir_item_name == bin_name
    //     })
    //     .map(|item| item.unwrap());

    // if let Some(path) = found {
    //     let metadata = fs::metadata(path.path()).expect("Error while reading metadata");
    //     let mode = metadata.permissions().mode();
    //     let is_executable = (mode & 0o111) != 0;
    //     if is_executable {
    //         println!("{bin_name} is {}", path.path().to_str().unwrap())
    //     }
    // } else {
    //     println!("{bin_name}: not found");
    // }
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
            Commands::TypeCommand => {
                if Commands::is_built(&rest_of_arguments) {
                    println!("{rest_of_arguments} is a shell builtin");
                } else {
                    let path_value = match env::var_os("PATH") {
                        Some(v) => v,
                        None => {
                            println!("No path env available");
                            continue;
                        }
                    };
                    is_bin_in_path(path_value, rest_of_arguments);
                }
            }
            Commands::Echo => {
                println!("{rest_of_arguments}")
            }
            Commands::Exit => {
                break;
            }
        }
    }
}
