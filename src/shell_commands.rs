use std::ffi::OsString;
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs};

#[derive(PartialEq, Debug)]
pub enum Commands {
    TypeCommand(Command),
    Exit,
    Echo(Command),
}

pub enum CommandError {
    CommandConversion(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Command {
    pub directive: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(directive: String, args: Vec<String>) -> Self {
        Command { directive, args }
    }
}
impl Commands {
    pub fn from(cmd: &Command) -> Result<Commands, CommandError> {
        match cmd.directive.as_str() {
            "type" => Ok(Commands::TypeCommand(cmd.clone())),
            "exit" => Ok(Commands::Exit),
            "echo" => Ok(Commands::Echo(cmd.clone())),
            _ => Err(CommandError::CommandConversion(cmd.directive.clone())),
        }
    }
    pub fn is_builtin(cmd: &Command) -> bool {
        Commands::from(&cmd).is_ok()
    }
    pub fn read_path() -> OsString {
        env::var_os("PATH").expect("PATH variable is not available")
    }
    fn is_file_executable(path: &Path) -> bool {
        let metadata = fs::metadata(&path).expect("Error while reading metadata");
        let mode = metadata.permissions().mode();
        (mode & 0o111) != 0
    }
    pub fn search_for_bin(bin_name: &str) -> bool {
        let path_value = Commands::read_path();
        let list_of_directories = env::split_paths(&path_value);
        for dir in list_of_directories {
            match fs::read_dir(&dir) {
                Ok(entries) => {
                    let dir_items: Vec<DirEntry> = entries.filter_map(|e| e.ok()).collect();
                    let directive_item = dir_items
                        .into_iter()
                        .map(|item| {
                            let name = item.file_name().display().to_string();
                            return (item, name);
                        })
                        .find(|(_, name)| name == bin_name)
                        .map(|(item, _)| item);
                    if let Some(executable) = directive_item {
                        if Commands::is_file_executable(&executable.path()) {
                            // println!("{} is {}", bin_name, executable.path().display());
                            return true;
                        }
                    } else {
                        continue;
                    }
                }
                Err(e) => eprintln!("Could not read the directory {}", e),
            }
        }
        println!("{}: not found", bin_name);
        return false;
    }
}
