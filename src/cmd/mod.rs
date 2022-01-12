use std::process::{Command, Output};

pub mod list;
pub mod view;

fn gh(args: &[&str]) -> Output {
    Command::new("gh")
        .args(args)
        .output()
        .expect("Failed to execute gh command")
}

// TODO: consider the case where the `open` command cannot be found
fn open(args: &[&str]) -> Output {
    Command::new("open")
        .args(args)
        .output()
        .expect("Failed to execute open command")
}
