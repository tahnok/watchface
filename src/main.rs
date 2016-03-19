extern crate regex;

use regex::Regex;
use std::str;
use std::process::Command;

fn main() {
    println!("Hello, world!");

    let new_cursor = connections("");
    println!("{}", new_cursor);
}

fn connections(cursor: &str) -> String {
    let re = Regex::new("User \"(.+)\" registered").unwrap();
    let mut command = Command::new("journalctl");

    command.arg("_COMM=ngircd")
           .arg("-o")
           .arg("cat")
           .arg("--no-pager")
           .arg("--show-cursor");

    if !cursor.is_empty() {
        command.arg("--after-cursor=")
               .arg(cursor);
    }

    let output = command.output().unwrap();

    println!("status: {}", output.status);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let logs = str::from_utf8(&output.stdout).unwrap();
    for user in re.captures_iter(logs) {
        println!("{}", user.at(1).unwrap_or("??"));
    }

    String::from(logs.lines().last().unwrap()).clone()
}
