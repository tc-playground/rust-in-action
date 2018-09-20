use std::io;
use std::process::{Child, Command, Stdio};
use std::io::{BufRead, BufReader};

fn main() {

    println!("Execute shell command:");
    let mut input = String::new();
    let err = io::stdin().read_line(&mut input);
    match err {
        Err(err) => panic!("Error reading input: '{}'", err),
        _ => ()
    };

    let parts: Vec<&str> = input.trim().split([' '].as_ref()).collect();
    let prog = parts[0];
    let args = &parts[1..];

    let cmd: io::Result<Child> = Command::new(prog)
            .args(args)
            .stdout(Stdio::piped())
            .spawn();

    let child = match cmd {
        Ok(mut child) => child,
        Err(err) => panic!("Failed to execute cmd: '{}' <- {}", prog, err),
    };

    let reader = BufReader::new(child.stdout.unwrap());
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    // let stdout_opt = child.stdout;
    // let stdout = stdout_opt.unwrap();
    // for line in BufReader::new(stdout).lines() {
    //     let data = line.unwrap();
    //     println!("{}", data);
    //     let bytes = data.as_bytes();
    //     writer.write_all(bytes).unwrap();
    // }
}