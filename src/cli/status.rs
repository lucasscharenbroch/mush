use crate::cli::{ExitType, MushSubcommand};
use crate::cli_expect;
use crate::index::status::IndexStatus;
use crate::io::{cwd_iter, read_index};
use colored::Colorize;

#[derive(clap::Args)]
pub struct StatusArgs {
}

impl MushSubcommand for StatusArgs {
    fn execute(&self) -> ExitType {
        let index = cli_expect!(read_index(), "read index")
            .unwrap_or(crate::index::Index::empty());

		let working_tree = cli_expect!(cwd_iter(), "read working tree");

		let index_status = cli_expect!(IndexStatus::create_from_index_and_working_tree(index, working_tree));

		if index_status.staged_changes.len() > 0 {
			println!("Changes to be committed:");
			index_status.staged_changes.iter()
				.for_each(|(action, file)| {
					println!(
						"    {}",
						format!("{action}: {file}").green()
					)
				});
			println!("");
		}

		if index_status.unstaged_changes.len() > 0 {
			println!("Changes not staged for commit:");
			index_status.unstaged_changes.iter()
				.for_each(|(action, file)| {
					println!(
						"    {}",
						format!("{action}: {file}").red()
					)
				});
			println!("");
		}

		if index_status.untracked_files.len() > 0 {
			println!("Untracked files:");
			index_status.untracked_files.iter()
				.for_each(|file| {
					println!(
						"    {}",
						file.red()
					)
				});
			println!("");
		}

        ExitType::Ok
    }
}
