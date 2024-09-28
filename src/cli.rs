mod init;
mod hash_object;
mod cat_file;

use hash_object::HashObjectArgs;
use init::InitArgs;
use cat_file::CatFileArgs;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

pub enum ExitType {
    Ok,
    Fatal,
}

impl Into<ExitCode> for ExitType {
    fn into(self) -> ExitCode {
        match self {
            Self::Ok => ExitCode::SUCCESS,
            Self::Fatal => ExitCode::from(128),
        }
    }
}

#[derive(Parser)]
#[command(version, about)]
#[command(name = crate::PROGRAM_NAME)]
#[command(version = crate::SEMANTIC_VERSION)]
#[command(about = crate::PROGRAM_DESCRIPTION)]
pub struct CliArgs {
    #[command(subcommand)]
    pub subcommand: CliSubcommand,
}

#[derive(Subcommand)]
pub enum CliSubcommand {
    /// Create an empty Mush repository in the current directory
    Init(InitArgs),
    /// Compute the hash of a file, optionally creating an object
    HashObject(HashObjectArgs),
    /// Provide contents or details of repository objects
    CatFile(CatFileArgs),
}

pub trait MushSubcommand {
    fn execute(&self) -> ExitType;
}

impl std::ops::Deref for CliSubcommand {
    type Target = dyn MushSubcommand;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Init(args) => args,
            Self::HashObject(args) => args,
            Self::CatFile(args) => args,
        }
    }
}

#[macro_export]
macro_rules! cli_expect {
    ($result:expr) => {
        match $result {
            Err(message) => {
                eprintln!("{}", message);
                return ExitType::Fatal;
            }
            Ok(x) => x,
        }
    };
}
