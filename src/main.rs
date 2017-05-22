use std::process::Command;
mod parser;

fn main() {
    Command::new("python").arg("-m").arg("main.py").output().expect("fail");
}
