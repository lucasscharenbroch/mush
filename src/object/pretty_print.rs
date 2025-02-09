use crate::{cli::CliResult, io::read_object_header};

use super::Object;

impl<'b> Object<'b> {
    /// Note that this is costly because it looks up object types from the mush database
    /// (for tree objects)
    pub fn pretty_print(&self) -> CliResult<String> {
        match self {
            Self::Blob(bytes) => Ok(String::from_utf8_lossy(bytes).to_string()),
            Self::Tree(entries) =>
                entries.iter()
                    .map(|entry| {
                        let object_type = read_object_header(&entry.hash)?.tipe;

                        Ok(format!(
                            "{:06} {} {}\t{}\n",
                            entry.mode,
                            object_type.to_str(),
                            entry.hash.as_str(),
                            entry.filename,
                        ))
                    })
                    .collect::<CliResult<String>>(),
            Self::Commit(commit_object) => Ok(commit_object.to_string()),
        }
    }
}
