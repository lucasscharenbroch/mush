//< This is written in a totally different style from git's `git config`.
//< Whipped up as a quick-and-dirty way to have a user-interface
//< for setting config without manual file manipulation.

use crate::{cli_expect, config::{read_config_option, write_config_option}};

use super::{ExitType, MushSubcommand};

#[derive(clap::Args)]
pub struct ConfigArgs {
    /// The option to get or set, e.g. `user.email`
    option: String,
    /// The new value for that option
    new_value: Option<String>,
}

impl MushSubcommand for ConfigArgs {
    fn execute(&self) -> ExitType {
        match self.new_value.as_ref() {
            None => { // read
                let current_val_opt = cli_expect!(
                    read_config_option(&self.option),
                    "read config option"
                );

                match current_val_opt {
                    Some(current_val) => println!("{current_val}"),
                    None => return ExitType::Fatal, // not found
                }
            },
            Some(new_val) => { // write
                cli_expect!(
                    write_config_option(&self.option, new_val),
                    "write config option"
                );
            }
        }

        ExitType::Ok
    }
}
