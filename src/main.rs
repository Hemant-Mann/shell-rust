use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

enum Command {
    Echo,
    Exit,
    Type,
    Executable,
    None,
}

fn identify_command(command: &str) -> Command {
    if command.trim().to_string() == "exit" {
        return Command::Exit;
    }
    if command.starts_with("echo ") {
        return Command::Echo;
    }
    if command.starts_with("type ") {
        return Command::Type;
    }
    return Command::None;
}

fn identify_sub_command(command: &str, path_env: &str) -> (Command, String) {
    let cmd = command.trim().to_string();
    return match cmd.as_str() {
        "exit" => (Command::Exit, "".to_owned()),
        "echo" => (Command::Echo, "".to_owned()),
        "type" => (Command::Type, "".to_owned()),
        _ => {
            let path_parts = if cfg!(target_os = "windows") {
                path_env.split(';')
            } else {
                path_env.split(':')
            };
            for path_dir in path_parts {
                if is_executeable(&path_dir, &cmd) {
                    let full_path = PathBuf::from(&path_dir)
                        .join(&cmd)
                        .to_str()
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    return (Command::Executable, full_path);
                }
            }
            return (Command::None, "".to_owned());
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

        let cmd = identify_command(&command);
        match cmd {
            Command::Exit => break,
            Command::Echo => print!("{}", &command[5..]),
            Command::None => println!("{}: command not found", command.trim()),
            Command::Type => {
                let sub_command = &command[5..];
                let (sub_cmd, cmd_path) = identify_sub_command(sub_command, &path_env);

                match sub_cmd {
                    Command::Echo | Command::Exit | Command::Type => {
                        println!("{} is a shell builtin", sub_command.trim())
                    }
                    Command::Executable => {
                        println!("{} is {}", sub_command.trim(), cmd_path)
                    }
                    Command::None => println!("{}: not found", sub_command.trim()),
                }
            }
            Command::Executable => {}
        }
    }
}
