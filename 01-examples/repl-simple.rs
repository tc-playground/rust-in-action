use std::io;
use std::process::{Command};
use std::io::{Write};

pub fn repl() -> io::Result<()> {
    loop {
        print!("repl> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let cmd = input.trim();
        if cmd == "exit" {
            return Ok(());
        }

        let child = Command::new("bash")
                .arg("-c")
                .arg(cmd)
                .spawn();

        match child {
            Ok(mut process) => {
                process.wait().unwrap();
            },
            Err(err) => println!("Failed to execute cmd: '{}' <- {}", cmd, err),
        };
    }
}

fn main() -> io::Result<()> {
    return repl();
}