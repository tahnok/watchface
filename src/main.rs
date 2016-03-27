extern crate irc;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use irc::client::prelude::*;
use regex::Regex;
use std::str;
use std::collections::HashSet;
use std::process;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    static ref USER: Regex = Regex::new(r#"User "(.+)!(.+)" registered"#).unwrap();
}

fn main() {
    let config = Config {
        nickname: Some(format!("bott")),
        server: Some(format!("127.0.0.1")),
        channels: Some(vec![format!("#main")]),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();

    let mut cursor = String::new();
    loop {
        let (new_cursor, nicks) = get_nicks(&cursor);
        cursor = new_cursor;
        for user in nicks.iter() {
            server.send_privmsg("wes", format!("{} has joined", user).as_str());
            println!("{}", user);
        }
        sleep(Duration::new(5,0));
    }
}

fn get_nicks(cursor: &String) -> (String, HashSet<String>) {
    let mut command = process::Command::new("journalctl");

    command.arg("_COMM=ngircd")
           .arg("-o")
           .arg("cat")
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
    for user in USER.captures_iter(logs) {
        let nick = user.at(1).unwrap();
        users.insert(String::from(nick));
    }

    (String::from(&logs.lines().last().unwrap()[11..]).clone(), users)
}
