use octocrab::models::Project;
use std::io::Error;
use std::process::{Command, Output};
use std::str::{self};
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
        Cmd::List { web } => list_prj(),
        Cmd::View { web } => view_prj(),
    }
}

fn list_prj() {
    let result = gh(&["api", "/repos/{owner}/{repo}/projects"]);
    let projects = extract_projects(result);
    if projects.len() != 0 {
        let project = &projects[0];
        println!("{:#?}", project.number);
    } else {
        // TODO: display owner/repo
        println!("This repository does not found any projects")
    }
}

fn view_prj() {
    // TODO
    println!("Not implemented view command");
}

fn extract_projects(result: Output) -> Vec<Project> {
    let stdout = str::from_utf8(&result.stdout).expect("Failed to covert str from API result");
    let projects =
        serde_json::from_reader(stdout.as_bytes()).expect("Failed to deserialize API result");
    projects
}

fn gh(args: &[&str]) -> Output {
    Command::new("gh")
        .args(args)
        .output()
        .expect("Failed to execute gh command")
}
