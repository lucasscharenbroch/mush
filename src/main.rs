mod cli;
mod hash;
mod io;
mod object;
mod refs;
mod revision; // plural to avoid name collision with `ref` keyword

use clap::Parser;

const SEMANTIC_VERSION: &'static str = "1.0";
const PROGRAM_NAME: &'static str = "mush";
const PROGRAM_DESCRIPTION: &'static str = "A minimalist git clone";

fn main() -> std::process::ExitCode {
    cli::CliArgs::parse().subcommand.execute().into()
}
