use crate::cli::{ExitType, MushSubcommand};
use crate::cli_expect;
use crate::io::read_index;
use crate::object::tree::FilenameTree;

#[derive(clap::Args)]
pub struct WriteTreeArgs {
}

impl MushSubcommand for WriteTreeArgs {
    fn execute(&self) -> ExitType {
        let index = cli_expect!(read_index(), "read index")
            .unwrap_or(crate::index::Index::empty());

        let object_tree = cli_expect!(FilenameTree::from_index(index).into_object_tree());

        cli_expect!(object_tree.write());

        println!("{}", object_tree.root().hash().as_str());

        ExitType::Ok
    }
}
