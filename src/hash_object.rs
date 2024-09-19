use clap::Args;

use crate::MushSubcommand;
use crate::cli_helpers::ExitType;

#[derive(Args)]
pub struct HashObjectArgs {
    /// Actually write the object into the object database
    #[arg(short)]
    write: bool,
    /// Use '-' for stdin
    file: String,
}

impl MushSubcommand for HashObjectArgs {
    fn execute(&self) -> ExitType {
        ExitType::Ok
    }
}
