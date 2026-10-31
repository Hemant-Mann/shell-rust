#[allow(unused_imports)]
use std::io::{self, Write};

enum Command {
    Echo,
    Exit,
    Type,
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

fn identify_sub_command(command: &str) -> Command {
    if command.trim().to_string() == "exit" {
        return Command::Exit;
    }
    if command.trim().to_string() == "echo" {
        return Command::Echo;
    }
    if command.trim().to_string() == "type" {
        return Command::Type;
    }
    return Command::None;
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let cmd = identify_command(&command);
        match cmd {
            Command::Exit => break,
            Command::Echo => print!("{}", &command[5..]),
            Command::Type => {
                let sub_command = &command[5..];
                let sub_cmd = identify_sub_command(sub_command);
                // println!("subcommand: {}", &command[5..]);
                match sub_cmd {
                    Command::Echo | Command::Exit | Command::Type => {
                        println!("{} is a shell builtin", sub_command.trim())
                    }
                    Command::None => println!("{}: not found", sub_command.trim()),
                }
            }
            Command::None => println!("{}: command not found", command.trim()),
        }
    }
}
