use crate::cli::ExitType;
use crate::cli::MushSubcommand;
use crate::io::create_file_all_no_overwrite;
use crate::io::dot_mush_slash;
use crate::io::read_filename_or_stdin_to_str;
use crate::object::Object;

use std::borrow::Cow;

#[derive(clap::Args)]
pub struct HashObjectArgs {
    /// Actually write the object into the object database
    #[arg(short)]
    write_result_to_database: bool,
    /// Use '-' for stdin
    filename: String,
}

impl MushSubcommand for HashObjectArgs {
    fn execute(&self) -> ExitType {
        let content =
            crate::cli_expect!(read_filename_or_stdin_to_str(&self.filename), "compute hash of object");
        let object = Object::Blob(Cow::Borrowed(content.as_bytes()));
        let hash = object.hash();

        println!("{}", hash.as_str()); // Not a debug print

        if self.write_result_to_database {
            let target_file = crate::cli_expect!(dot_mush_slash(&object.hash().path()), "resolve path");
            crate::cli_expect!(
                create_file_all_no_overwrite(&target_file, object.compressed().as_slice()),
                "write hash-object"
            );
        }

        ExitType::Ok
    }
}
