use std::process::{Command, Output};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "prj")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub cmd: Cmd,
}

#[derive(StructOpt, Debug)]
pub enum Cmd {
    /// List Projects in this repository
    List {
        #[structopt(short = "w", long)]
        web: bool,
    },

    /// Display the information about a Project
    View {
        #[structopt(short = "w", long)]
        web: bool,
    },
}

pub fn exec_cmd(args: CommandLineArgs) {
    let CommandLineArgs { cmd } = args;
    match cmd {
        List => list_prj(),
        View => {}
    }
}

fn list_prj() {
    let result = gh(&["api", "/repos/{owner}/{repo}/projects"]);
    println!("{:#?}", result);
}

fn gh(args: &[&str]) -> Output {
    Command::new("gh")
        .args(args)
        .output()
        .expect("Failed to execute gh command")
}
