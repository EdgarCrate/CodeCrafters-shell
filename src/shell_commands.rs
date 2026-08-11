use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(PartialEq, Debug)]
pub enum AvailableCommands {
    TypeCommand,
    Exit,
    Echo,
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
    pub fn from(directive: &str) -> Result<AvailableCommands, CommandError> {
        match directive {
            "type" => Ok(AvailableCommands::TypeCommand),
            "exit" => Ok(AvailableCommands::Exit),
            "echo" => Ok(AvailableCommands::Echo),
            _ => Err(CommandError::CommandConversion),
        }
    }
    pub fn run_external_process(cmd: &Command, cwd: &PathBuf) -> String {
        std::process::Command::new(&cmd.directive)
            .args(&cmd.args)
            .current_dir(cwd)
            .output()
            .map(|output| {
                let out_bytes = if output.status.success() {
                    output.stdout
                } else {
                    output.stderr
                };
                String::from_utf8_lossy(&out_bytes)
                    .trim_end_matches('\n') // strip the program's own trailing newline
                    .to_owned()
            })
            .unwrap_or_else(|e| format!("Failed to execute program: {}", e))
    }
    pub fn is_builtin(directive: &str) -> bool {
        AvailableCommands::from(directive).is_ok()
    }
    pub fn read_path() -> OsString {
        env::var_os("PATH").expect("PATH variable is not available")
    }
    fn is_file_executable(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(path) {
                // Check if any execute bit (owner, group, other) is set
                return metadata.permissions().mode() & 0o111 != 0;
            }
            false
        }

        #[cfg(windows)]
        {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                return ext_lower == "exe"
                    || ext_lower == "bat"
                    || ext_lower == "cmd"
                    || ext_lower == "com";
            }
            false
        }
    }
    pub fn search_for_bin(bin_name: &str) -> Option<PathBuf> {
        let path_value = AvailableCommands::read_path();
        let list_of_directories = env::split_paths(&path_value);

        // Get extensions to try, e.g. [".COM", ".EXE", ".BAT", ".CMD"]
        let pathext = env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string());
        let extensions: Vec<String> = pathext.split(';').map(|s| s.to_lowercase()).collect();

        for dir in list_of_directories {
            let entries = match fs::read_dir(&dir) {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("Could not read the directory {}", err);
                    continue;
                }
            };

            for entry in entries.filter_map(|e| e.ok()) {
                let file_name = entry.file_name().to_string_lossy().to_lowercase();

                // Case A: bin_name already includes an extension, e.g. "python.exe"
                if file_name == bin_name.to_lowercase()
                    && AvailableCommands::is_file_executable(&entry.path())
                {
                    return Some(entry.path());
                }

                // Case B: bin_name is bare, e.g. "python" — try appending PATHEXT entries
                for ext in &extensions {
                    if file_name == format!("{}{}", bin_name.to_lowercase(), ext) {
                        let path = entry.path();
                        if AvailableCommands::is_file_executable(&path) {
                            return Some(path);
                        }
                    }
                }
            }
        }
        None
    }
}
