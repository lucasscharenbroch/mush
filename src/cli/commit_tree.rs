use crate::cli::{ExitType, MushSubcommand};
use crate::cli_expect;
use crate::config::force_get_user;
use crate::io::{read_object_header, read_stdin_to_str, write_object};
use crate::object::commit::CommitObject;
use crate::object::ObjectType;
use crate::revision::RevisionSpec;

#[derive(clap::Args)]
pub struct CommitTreeArgs {
    /// Hash of the tree to commit
    tree: String,
}

impl MushSubcommand for CommitTreeArgs {
    fn execute(&self) -> ExitType {
        // probably dont't need to allow full refs syntax here, but whatever.
        let revision_spec = crate::cli_expect!(RevisionSpec::parse(&self.tree));
        let hash = crate::cli_expect!(revision_spec.dereference());
        let header = crate::cli_expect!(read_object_header(&hash));

        if header.tipe != ObjectType::Tree {
            crate::cli_panic!(format!("Not a tree: {}", hash.to_string()));
        }

        let user = crate::cli_expect!(force_get_user());
        let message = crate::cli_expect!(read_stdin_to_str(), "get commit message");
        let object = CommitObject::new(hash, Vec::new(), user, message).into();
        cli_expect!(write_object(&object));

        println!("{}", object.hash().to_string());

        ExitType::Ok
    }
}
