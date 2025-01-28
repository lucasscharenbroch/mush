use crate::cli::{ExitType, MushSubcommand};
use crate::io::{create_directories_no_overwrite, create_file_no_overwrite};

#[derive(clap::Args)]
pub struct InitArgs {}

impl MushSubcommand for InitArgs {
    fn execute(&self) -> ExitType {
        const REASON: &'static str = "initialize repo";
        const DEFAULT_CONFIG: &'static str = "[[default-config-placeholder]]";
        const DEFAULT_HEAD: &'static str = "[[default-head-placeholder]]";

        crate::cli_expect!(
            create_directories_no_overwrite(["./.mush", "./.mush/objects", "./.mush/refs"].iter()),
            REASON
        );

        crate::cli_expect!(create_file_no_overwrite("./.mush/config", DEFAULT_CONFIG.as_bytes()), REASON);
        crate::cli_expect!(create_file_no_overwrite("./.mush/HEAD", DEFAULT_HEAD.as_bytes()), REASON);

        ExitType::Ok
    }
}
