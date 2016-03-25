#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::str;
use std::collections::HashSet;
use std::process::Command;
use std::process::Output;

lazy_static! {
    static ref USER: Regex = Regex::new(r#"User "(.+)!(.+)" registered"#).unwrap();
}

fn main() {
    let (new_cursor, users) = connections("");
    for user in users.iter() {
        println!("{}", user);
    }
    connections(&new_cursor);
}

fn connections(cursor: &str) -> (String, HashSet<String>) {
    let mut command = Command::new("journalctl");

    command.arg("_COMM=ngircd")
           .arg("-o")
           .arg("cat")
           .arg("--no-pager")
           .arg("--show-cursor");

    if !cursor.is_empty() {
        command.arg(format!("--after-cursor={}", cursor));
    }

    let Output {status, stdout, stderr} = command.output().unwrap();

    if !status.success() {
        println!("status: {}", status);
        println!("stderr: {}", str::from_utf8(&stderr).unwrap());
        panic!();
    }

    let logs = str::from_utf8(&stdout).unwrap();
    let mut users = HashSet::new();
    for user in USER.captures_iter(logs) {
        let nick = user.at(1).unwrap();
        users.insert(String::from(nick));
    }

    (String::from(&logs.lines().last().unwrap()[11..]).clone(), users)
}
