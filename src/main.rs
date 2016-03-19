extern crate regex;

use regex::Regex;
use std::str;
use std::process::Command;
use std::process::Output;

fn main() {
    println!("Hello, world!");

    let re = Regex::new("User \"(.+)\" registered").unwrap();
    let new_cursor = connections("", &re);
    println!("{}", new_cursor);
    let new_cursor = connections(&new_cursor, &re);
    println!("{}", new_cursor);
}

fn connections(cursor: &str, re: &Regex) -> String {
    let mut command = Command::new("journalctl");

    command.arg("_COMM=ngircd")
           .arg("-o")
           .arg("cat")
           .arg("--no-pager")
           .arg("--show-cursor");

    if !cursor.is_empty() {
        command.arg(format!("--after-cursor={}", cursor));
    }

    println!("command {:?}", command);
    let Output {status, stdout, stderr} = command.output().unwrap();

    if !status.success() {
        println!("status: {}", status);
        println!("stderr: {}", str::from_utf8(&stderr).unwrap());
        panic!();
    }

    let logs = str::from_utf8(&stdout).unwrap();
    for user in re.captures_iter(logs) {
        println!("{}", user.at(1).unwrap_or("??"));
    }

    String::from(&logs.lines().last().unwrap()[11..]).clone()
}
