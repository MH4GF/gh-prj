use crate::cmd::{list, view};
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
        Cmd::List { .. } => list::list_prj(),
        Cmd::View { web_mode, arg } => view::view_prj(web_mode, arg),
    }
}
