mod cat_file;
mod hash_object;
mod init;
mod update_index;
mod write_tree;
mod commit_tree;

use cat_file::CatFileArgs;
use commit_tree::CommitTreeArgs;
use hash_object::HashObjectArgs;
use init::InitArgs;

use clap::{Parser, Subcommand};
use update_index::UpdateIndexArgs;
use write_tree::WriteTreeArgs;
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
    // descriptions copied/inspired by git manpages

    /// Create an empty Mush repository in the current directory
    Init(InitArgs),
    /// Compute the hash of a file, optionally creating an object
    HashObject(HashObjectArgs),
    /// Provide contents or details of repository objects
    CatFile(CatFileArgs),
    /// Register file contents in the working tree to the index
    UpdateIndex(UpdateIndexArgs),
    /// Create a tree object from the current index
    WriteTree(WriteTreeArgs),
    /// Create a new commit object
    CommitTree(CommitTreeArgs),
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
            Self::UpdateIndex(args) => args,
            Self::WriteTree(args) => args,
            Self::CommitTree(args) => args,
        }
    }
}

pub type CliResult<T> = Result<T, String>;
pub type ContextlessCliResult<T> = Result<T, Box<dyn FnOnce(&str) -> String>>;

pub fn with_context<T>(context: &str, result: ContextlessCliResult<T>) -> CliResult<T> {
    result.map_err(|callback| callback(context))
}

#[macro_export]
macro_rules! cli_panic {
    ($message:expr /* CliResult<T> */) => { /* -> ExitType? */
        eprintln!("{}", $message);
        return ExitType::Fatal;
    }
}

#[macro_export]
macro_rules! cli_expect {
    ($result:expr /* CliResult<T> */) => { /* -> ExitType? */
        match $result {
            Err(message) => {
                crate::cli_panic!(message);
            }
            Ok(x) => x,
        }
    };

    ($result:expr /* ContextlessCliResult<T> */, $reason:expr /* &str */) => { /* -> ExitType? */
        match $result {
            Err(callback) => {
                eprintln!("{}", callback($reason));
                return ExitType::Fatal;
            }
            Ok(x) => x,
        }
    };
}
