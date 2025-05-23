use crate::cli::ExitType;
use crate::cli::MushSubcommand;
use crate::cli_expect;
use crate::io::read_filename_or_stdin_to_str;
use crate::io::write_object;
use crate::object::Object;

use std::borrow::Cow;

#[derive(clap::Args)]
pub struct HashObjectArgs {
    /// Actually write the object into the object database
    #[arg(short)]
    write_result_to_database: bool,
    /// Use '-' for stdin
    filenames: Vec<String>,
}

impl MushSubcommand for HashObjectArgs {
    fn execute(&self) -> ExitType {
        for filename in self.filenames.iter() {
            let content =
                crate::cli_expect!(read_filename_or_stdin_to_str(filename), "compute hash of object");
            let object = Object::Blob(Cow::Borrowed(content.as_bytes()));
            let hash = object.hash();

            println!("{}", hash.as_str()); // Not a debug print

            if self.write_result_to_database {
                cli_expect!(write_object(&object))
            }
        }

        ExitType::Ok
    }
}
