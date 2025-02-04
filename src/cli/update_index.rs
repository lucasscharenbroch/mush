use crate::cli::ExitType;
use crate::cli::MushSubcommand;
use crate::cli_expect;
use crate::hash::Hash;
use crate::io::repo_canononicalize;
use crate::index::Index;
use crate::index::IndexEntry;
use crate::io::create_file;
use crate::io::dot_mush_slash;
use crate::io::file_metadata;
use crate::io::read_index;

#[derive(clap::Args)]
pub struct UpdateIndexArgs {
    #[command(flatten)]
    action: UpdateIndexActionArgs,

    #[arg(requires = "action")]
    file: String,
}

#[derive(Clone)]
enum UpdateIndexAction {
    Add(String),
    Remove, // analagous to `git --force-remove`
}

/// UpdateIndexAction (as mutually exclusive flags):
#[derive(clap::Args)]
#[group(required = true)]
struct UpdateIndexActionArgs {
    #[arg(long, group = "action", value_name = "HASH")]
    add: Option<String>,
    #[arg(long, group = "action")]
    remove: bool,
}

impl UpdateIndexActionArgs {
    fn to_enum(&self) -> UpdateIndexAction {
        match (self.add.as_ref(), self.remove) {
            (Some(hash), false) => UpdateIndexAction::Add(hash.clone()),
            (None, true) => UpdateIndexAction::Remove,
            _ => panic!("Clap invariant violated: args not mutually exclusive"),
        }
    }
}

impl MushSubcommand for UpdateIndexArgs {
    fn execute(&self) -> ExitType {
        let index_file_name = cli_expect!(dot_mush_slash("index"), "resolve path");
        let mut index = cli_expect!(read_index(), "update index")
            .unwrap_or(Index::new()) ;

        match self.action.to_enum() {
            UpdateIndexAction::Add(hash) => {
                let metadata = cli_expect!(file_metadata(&self.file), "read file metadata");
                let filename = cli_expect!(repo_canononicalize(&self.file), "canonicalize filename");
                let hash = cli_expect!(
                    Hash::try_from_str(&hash)
                        .ok_or(format!("Bad hash: `{}`", hash))
                );

                index.entries().insert(
                    filename.clone(),
                    IndexEntry::new(filename, hash, metadata)
                );

            },
            UpdateIndexAction::Remove => {
                let filename = cli_expect!(repo_canononicalize(&self.file), "canonicalize filename");

                cli_expect!(
                    index.entries().remove(&filename)
                        .ok_or(format!("No index entry for {filename}"))
                );
            }
        }

        cli_expect!(
            create_file(&index_file_name, index.serialize().as_slice()),
            "write index"
        );

        ExitType::Ok
    }
}
