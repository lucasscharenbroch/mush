mod io;
mod cli;
mod object;
mod hash;

use clap::Parser;

const SEMANTIC_VERSION: &'static str = "1.0";
const PROGRAM_NAME: &'static str = "mush";
const PROGRAM_DESCRIPTION: &'static str = "A minimalist git clone";

fn main() -> std::process::ExitCode {
    cli::CliArgs::parse().subcommand.execute().into()
}
