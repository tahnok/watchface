extern crate irc;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use irc::client::prelude::*;
use regex::Regex;
use std::{str, process};
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    static ref USER_REGISTER_RE: Regex = Regex::new(r#"User "(.+)!(.+)" registered"#).unwrap();
    static ref DELAY: Duration = Duration::new(5,0);
}
static CURSOR_OFFSET: usize = 11; // '-- cursor: '
static OWNER: str = *"wes";

fn main() {
    let server = IrcServer::new("irc.json").unwrap();
    server.identify().unwrap();

    let mut cursor = String::new();
    loop {
        let (new_cursor, nicks) = get_nicks(&cursor);
        cursor = new_cursor;
        for user in nicks.iter() {
            let resp = server.send_privmsg(&OWNER, format!("{} has joined", user).as_str());
            if resp.is_err() {
                println!("failed to send");
            }
            println!("{}", user);
        }
        sleep(*DELAY);
    }
}

fn get_nicks(cursor: &String) -> (String, HashSet<String>) {
    let mut command = process::Command::new("journalctl");

    command.arg("_COMM=ngircd")
           .arg("-o").arg("cat")
           .arg("--no-pager")
           .arg("--show-cursor");

    if !cursor.is_empty() {
        command.arg(format!("--after-cursor={}", cursor));
    }

    let process::Output {status, stdout, stderr} = command.output().unwrap();

    if !status.success() {
        println!("status: {}", status);
        println!("stderr: {}", str::from_utf8(&stderr).unwrap());
        panic!();
    }

    let logs = str::from_utf8(&stdout).unwrap();
    let mut users = HashSet::new();
    for user in USER_REGISTER_RE.captures_iter(logs) {
        let nick = user.at(1).unwrap();
        users.insert(String::from(nick));
    }

    let new_cursor = String::from(&logs.lines().last().unwrap()[CURSOR_OFFSET..]).clone();
    (new_cursor, users)
}
