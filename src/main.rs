use structopt::StructOpt;
mod cli;
mod cmd;
use cli::CommandLineArgs;

fn main() {
    let args = CommandLineArgs::from_args();
    cli::exec_cmd(args)
}
