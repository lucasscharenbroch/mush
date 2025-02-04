use crate::{cli::{ExitType, MushSubcommand}, cli_expect, io::{read_index, write_object}, object::Object};

#[derive(clap::Args)]
pub struct WriteTreeArgs {
}

impl MushSubcommand for WriteTreeArgs {
    fn execute(&self) -> ExitType {
        let index = cli_expect!(read_index(), "read index")
            .unwrap_or(crate::index::Index::empty());

        let entries = index.into_entries()
            .into_values()
            .map(|entry| entry.into())
            .collect::<Vec<_>>();

        let tree_object = Object::Tree(entries);

        cli_expect!(write_object(&tree_object));

        println!("{}", tree_object.hash().as_str());

        ExitType::Ok
    }
}
