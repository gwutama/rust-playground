use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    let out = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output()
        .expect("ls command failed to start");

    println!("status: {}", out.status);
    io::stdout().write_all(&out.stdout).unwrap();
}
