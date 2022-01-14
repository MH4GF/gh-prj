use gh_prj::cli;
use gh_prj::cli::CommandLineArgs;
use structopt::StructOpt;

fn main() {
    let args = CommandLineArgs::from_args();

    std::process::exit(match cli::exec_cmd(args) {
        true => 0,
        false => 1,
    });
}
