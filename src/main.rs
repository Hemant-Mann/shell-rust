#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        if command.trim().to_string() == "exit" {
            break;
        }
        if command.starts_with("echo ") {
            print!("{}", &command[5..]);
        } else {
            println!("{}: command not found", command.trim())
        }
    }
}
