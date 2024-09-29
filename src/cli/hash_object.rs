use crate::cli::MushSubcommand;
use crate::cli::ExitType;
use crate::object::Object;

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
        let content = crate::read_filename_or_stdin_to_str!(self.filename, "Compute hash of object");
        let object = Object::Blob(content.as_bytes());
        let hash = object.hash();

        println!("{}", hash.as_str()); // Not a debug print

        if self.write_result_to_database {
            let target_file = crate::dot_mush_slash!(object.hash().path());
            crate::create_file_all_no_overwrite!(
                &target_file,
                object.compressed().as_slice(),
                "write hash-object"
            );
        }

        ExitType::Ok
    }
}
