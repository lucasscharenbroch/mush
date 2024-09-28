use crate::cli::MushSubcommand;
use crate::cli::ExitType;
use crate::mush_object::MushObject;

#[derive(clap::Args)]
pub struct HashObjectArgs {
    /// Actually write the object into the object database
    #[arg(short)]
    write_result_to_database: bool,
    /// Use '-' for stdin
    file: String,
}

impl MushSubcommand for HashObjectArgs {
    fn execute(&self) -> ExitType {
        let content = crate::read_file_or_stdin!(self.file, "Compute hash of object");
        let object = MushObject::Blob(content.as_bytes());
        let hash = object.hash();

        println!("{}", hash.as_str());

        if self.write_result_to_database {
            let target_file = crate::dot_mush_slash!(object.object_path());
            crate::create_file_all_no_overwrite!(
                &target_file,
                object.compressed().as_slice(),
                "write hash-object"
            );
        }

        ExitType::Ok
    }
}
