use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(PartialEq, Debug)]
pub enum AvailableCommands {
    TypeCommand(Command),
    Exit,
    Echo(Vec<String>),
}

pub enum CommandError {
    CommandConversion,
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
impl AvailableCommands {
    pub fn from(cmd: &Command) -> Result<AvailableCommands, CommandError> {
        match cmd.directive.as_str() {
            "type" => Ok(AvailableCommands::TypeCommand(cmd.clone())),
            "exit" => Ok(AvailableCommands::Exit),
            "echo" => Ok(AvailableCommands::Echo(cmd.args.clone())),
            _ => Err(CommandError::CommandConversion),
        }
    }
    pub fn run_external_process(cmd: &Command, path: &PathBuf) -> String {
        std::process::Command::new(&cmd.directive)
            .args(&cmd.args)
            .current_dir(path)
            .output()
            .map(|output| {
                // Select stdout if successful, stderr if it failed
                let out_bytes = if output.status.success() {
                    output.stdout
                } else {
                    output.stderr
                };
                String::from_utf8_lossy(&out_bytes).into_owned()
            })
            .unwrap_or_else(|e| format!("Failed to execute program: {}", e))
    }
    pub fn is_builtin(cmd: &Command) -> bool {
        AvailableCommands::from(&cmd).is_ok()
    }
    pub fn read_path() -> OsString {
        env::var_os("PATH").expect("PATH variable is not available")
    }
    fn is_file_executable(path: &Path) -> bool {
        // 1. Ensure it is actually a file, not a directory
        if !path.is_file() {
            return false;
        }
        // 2. Check if the extension matches common Windows executables
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            return ext_lower == "exe"
                || ext_lower == "bat"
                || ext_lower == "cmd"
                || ext_lower == "com";
        }
        false
    }
    pub fn search_for_bin(bin_name: &str) -> Option<PathBuf> {
        let path_value = AvailableCommands::read_path();
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
                        if AvailableCommands::is_file_executable(&executable.path()) {
                            return Some(executable.path());
                        }
                    } else {
                        continue;
                    }
                }
                Err(e) => eprintln!("Could not read the directory {}", e),
            }
        }
        return None;
    }
}
