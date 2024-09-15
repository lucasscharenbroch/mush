mod init;

use init::InitArgs;

use clap::{Parser, Subcommand};

const SEMANTIC_VERSION: &'static str = "1.0";
const PROGRAM_NAME: &'static str = "mush";
const PROGRAM_DESCRIPTION: &'static str = "A minimalist git clone";

#[derive(Parser)]
#[command(version, about)]
#[command(name = crate::PROGRAM_NAME)]
#[command(version = crate::SEMANTIC_VERSION)]
#[command(about = crate::PROGRAM_DESCRIPTION)]
pub struct CliArgs {
    #[command(subcommand)]
    subcommand: CliSubcommand,
}


#[derive(Subcommand)]
enum CliSubcommand {
    /// Create an empty Mush repository in the current directory
    Init(InitArgs),
}

trait MushSubcommand {
    fn execute(&self);
}

impl std::ops::Deref for CliSubcommand {
    type Target = dyn MushSubcommand;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Init(args) => args,
        }
    }
}

fn main() {
    CliArgs::parse().subcommand.execute();
}
