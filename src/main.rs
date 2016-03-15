extern crate regex;

use regex::Regex;
use std::str;
use std::process::Command;

fn main() {
    println!("Hello, world!");
    let re = Regex::new("User \"(.+)\" registered").unwrap();

    // journalctl _COMM=ngircd -o cat --no-pager
    let output = Command::new("journalctl")
                         .arg("_COMM=ngircd")
                         .arg("-o")
                         .arg("cat")
                         .arg("--no-pager")
                         .output()
                         .unwrap_or_else(|e| {
                             panic!("failed to execute process: {}", e)
                         });

    println!("status: {}", output.status);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    for user in re.captures_iter(str::from_utf8(&output.stdout).unwrap()) {
        println!("{}", user.at(1).unwrap_or("??"));
    }
}
