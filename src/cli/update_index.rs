use crate::cli::ExitType;
use crate::cli::MushSubcommand;
use crate::cli_expect;
use crate::index::Index;
use crate::index::IndexEntry;
use crate::io::create_file;
use crate::io::dot_mush_slash;

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
        match self.action.to_enum() {
            UpdateIndexAction::Add(hash) => {
                let index_file_name = cli_expect!(dot_mush_slash("index"), "resolve path");

                let mut index = if std::path::Path::new(&index_file_name).exists() {
                    todo!("read, deserialize index from file")
                } else {
                    Index::new()
                };

                // TODO check if file already exists in index

                index.entries().push(
                    cli_expect!(IndexEntry::new(&self.file, &hash), "create index entry")
                );

                cli_expect!(
                    create_file(&index_file_name, index.serialize().as_slice()),
                    "write index"
                );
            },
            UpdateIndexAction::Remove => {
                todo!("implement remove")
            }
        }

        ExitType::Ok
    }
}
