mod init;
mod hash_object;

use hash_object::HashObjectArgs;
use init::InitArgs;

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
        }
    }
}