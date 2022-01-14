use gh_prj::cli;
use gh_prj::cli::CommandLineArgs;
use structopt::StructOpt;

fn main() {
    let args = CommandLineArgs::from_args();
    cli::exec_cmd(args)
}
