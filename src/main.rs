use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

enum CommandType {
    Echo,
    Exit,
    Type,
    Executable,
    None,
}

fn identify_command(command: &str, path_env: &str) -> CommandType {
    if command.trim().to_string() == "exit" {
        return CommandType::Exit;
    }
    if command.starts_with("echo ") {
        return CommandType::Echo;
    }
    if command.starts_with("type ") {
        return CommandType::Type;
    }
    // split the command and check if binary exists then execute the command by passing the args
    let binding = command.to_string();
    let parts: Vec<&str> = binding.split_whitespace().collect();
    let is_exec = is_command_executable(parts[0].to_string(), path_env);
    if is_exec.0 {
        return CommandType::Executable;
    }
    return CommandType::None;
}

fn is_command_executable(cmd: String, path_env: &str) -> (bool, String) {
    let path_parts = if cfg!(target_os = "windows") {
        path_env.split(';')
    } else {
        path_env.split(':')
    };
    for path_dir in path_parts {
        if is_executeable(&path_dir, &cmd) {
            let full_path = PathBuf::from(&path_dir)
                .join(cmd)
                .to_str()
                .map(|s| s.to_string())
                .unwrap_or_default();
            return (true, full_path);
        }
    }
    return (false, "".to_owned());
}

fn identify_sub_command(command: &str, path_env: &str) -> (CommandType, String) {
    let cmd = command.trim().to_string();
    return match cmd.as_str() {
        "exit" => (CommandType::Exit, "".to_owned()),
        "echo" => (CommandType::Echo, "".to_owned()),
        "type" => (CommandType::Type, "".to_owned()),
        _ => {
            let (is_exec, full_path) = is_command_executable(cmd, path_env);
            if is_exec {
                return (CommandType::Executable, full_path);
            }
            return (CommandType::None, "".to_owned());
        }
    };
}

fn is_executeable(dir_str: &str, file_name: &str) -> bool {
    let full_path = PathBuf::from(&dir_str).join(&file_name);
    let file = Path::new(&full_path);
    if !file.is_file() {
        return false;
    }

    let metadata = fs::metadata(file).expect("Failed to read metadata");
    let mode = metadata.permissions().mode();

    return mode & 0o111 != 0;
}

fn main() {
    let path_env = env::var("PATH").unwrap_or_else(|_| "".to_string());

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let cmd = identify_command(&command, &path_env);
        match cmd {
            CommandType::Exit => break,
            CommandType::Echo => print!("{}", &command[5..]),
            CommandType::None => println!("{}: command not found", command.trim()),
            CommandType::Type => {
                let sub_command = &command[5..];
                let (sub_cmd, cmd_path) = identify_sub_command(sub_command, &path_env);

                match sub_cmd {
                    CommandType::Echo | CommandType::Exit | CommandType::Type => {
                        println!("{} is a shell builtin", sub_command.trim())
                    }
                    CommandType::Executable => {
                        println!("{} is {}", sub_command.trim(), cmd_path)
                    }
                    CommandType::None => println!("{}: not found", sub_command.trim()),
                }
            }
            CommandType::Executable => {
                let binding = command.to_string();
                let parts: Vec<&str> = binding.split_whitespace().collect();
                let command_args = &parts[1..];

                if let Ok(output) = Command::new(parts[0]).args(command_args).output() {
                    if let Ok(text) = String::from_utf8(output.stdout) {
                        print!("{}", text)
                    }
                }
            }
        }
    }
}
