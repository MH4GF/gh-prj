use octocrab::models::Project;
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
    #[structopt(usage = "gh prj list [FLAGS]")]
    List {
        /// Open the browser to list the project(s)
        #[structopt(short = "w", long = "web")]
        web_mode: bool,
    },

    /// Display the information about a Project
    #[structopt(usage = "gh prj view <number> [FLAGS]")]
    View {
        /// issue number or url
        #[structopt(name = "number")]
        arg: isize,

        /// Open a project in the browser
        #[structopt(short = "w", long = "web")]
        web_mode: bool,
    },
}

pub fn exec_cmd(args: CommandLineArgs) {
    let CommandLineArgs { cmd } = args;
    match cmd {
        Cmd::List { .. } => list_prj(),
        Cmd::View { web_mode, arg } => view_prj(web_mode, arg),
    }
}

fn list_prj() {
    let result = gh(&["api", "/repos/{owner}/{repo}/projects"]);
    let projects = extract_projects(result);
    if projects.is_empty() {
        // TODO: display owner/repo
        println!("This repository does not found any projects");
        return;
    }

    for prj in projects {
        println!("{:#?}\n", prj.number);
    }
}

fn view_prj(web_mode: bool, arg: isize) {
    let query = format!(
        "limit(1; .[] | select(.number == {i}))",
        i = &arg.to_string()
    );
    let result = gh(&["api", "/repos/{owner}/{repo}/projects", "-q", &query]);
    let stdout = str::from_utf8(&result.stdout).expect("Failed to covert str from API result");
    if stdout.is_empty() {
        println!("Failed to execute gh command: {:?}", &result.stderr);
        return;
    }

    let project: Project =
        serde_json::from_reader(stdout.as_bytes()).expect("Failed to deserialize API result");

    if web_mode {
        let url = project.html_url;
        let url_str: String = url.into();
        open(&[&url_str]);
        println!("Opening {:?} in your browser.", &url_str);

        return;
    }

    // TODO
    println!("{:?}", project);
    println!("Not implemented view command");
}

fn extract_projects(result: Output) -> Vec<Project> {
    let stdout = str::from_utf8(&result.stdout).expect("Failed to covert str from API result");
    let projects =
        serde_json::from_reader(stdout.as_bytes()).expect("Failed to deserialize API result");
    projects
}

// TODO: consider the case where the `open` command cannot be found
fn open(args: &[&str]) -> Output {
    Command::new("open")
        .args(args)
        .output()
        .expect("Failed to execute open command")
}

fn gh(args: &[&str]) -> Output {
    Command::new("gh")
        .args(args)
        .output()
        .expect("Failed to execute gh command")
}
