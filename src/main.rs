use clap::Parser;

fn main() -> std::process::ExitCode {
    mush::cli::CliArgs::parse().subcommand.execute().into()
}
