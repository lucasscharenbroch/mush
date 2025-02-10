use std::path::Path;

use crate::cli::{ExitType, MushSubcommand};
use crate::io::{create_directories_no_overwrite, create_directory_all, create_file_no_overwrite};

#[derive(clap::Args)]
pub struct InitArgs {
    directory: Option<String>,
}

impl MushSubcommand for InitArgs {
    fn execute(&self) -> ExitType {
        const REASON: &'static str = "initialize repo";
        const DEFAULT_HEAD: &'static str = "[[default-head-placeholder]]";

        let directory = match self.directory {
            Some(ref dir) => {
                crate::cli_expect!(create_directory_all(dir), "create directory for repository");
                let path = Path::new(&dir).canonicalize().expect(&format!("Failed to canonicalize path: `{dir}`"));
                String::from(path.to_str().expect("Couldn't stringify path: `{path}`"))
            },
            None => {
                String::from(".")
            },
        };

        crate::cli_expect!(
            create_directories_no_overwrite(
                [
                    format!("{directory}/.mush").as_str(),
                    format!("{directory}/.mush/objects").as_str(),
                    format!("{directory}/.mush/refs").as_str(),
                    format!("{directory}/.mush/config").as_str(),
                ].iter()
            ),
            REASON
        );

        crate::cli_expect!(create_file_no_overwrite(&format!("{directory}/.mush/HEAD"), DEFAULT_HEAD.as_bytes()), REASON);

        ExitType::Ok
    }
}
