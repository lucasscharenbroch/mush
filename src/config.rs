use itertools::Itertools;

use crate::{cli::{with_context, CliResult, ContextlessCliResult}, io::{create_file_all, dot_mush_slash, file_exists, overwrite_file, read_filename_to_str, try_open_filename, try_read_filename_to_str}};

pub struct User {
    pub name: String,
    pub email: String,
}

pub struct PartialUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl PartialUser {
    fn try_into_user(self) -> Option<User> {
        Some(
            User {
                name: self.name?,
                email: self.email?,
            }
        )
    }
}

struct MushConfig {
    user: PartialUser,
}

//< Use a filesystem-based, per-repository (non-global) config.
//< Writing a global config would be nicer for actual use,
//< but it would be a hairier (though not much more interesting)
//< implementation, plus making testing much more tedious.
pub fn read_config_option(option_name: &str) -> ContextlessCliResult<Option<String>> {
    let target_file = dot_mush_slash(
        &format!("config/{}", option_name.split(".").join("/"))
    )?;

    try_read_filename_to_str(&target_file)
}

pub fn write_config_option(option_name: &str, value: &str) -> ContextlessCliResult<()> {
    let target_file = dot_mush_slash(
        &format!("config/{}", option_name.split(".").join("/"))
    )?;

    if file_exists(&target_file) {
        overwrite_file(&target_file, value.as_bytes())?;
    } else {
        create_file_all(&target_file, value.as_bytes())?;
    }

    Ok(())
}

impl MushConfig {
    fn _read() -> ContextlessCliResult<MushConfig> {
        Ok(
            MushConfig {
                user: PartialUser {
                    name: read_config_option("user.name")?,
                    email: read_config_option("user.email")?,
                }
            }
        )
    }

    fn read() -> CliResult<MushConfig> {
        with_context("read mush config", MushConfig::_read())
    }
}

pub fn force_get_user() -> CliResult<User> {
    let config = MushConfig::read()?;

    config.user.try_into_user()
        .ok_or(String::from(
            concat!(
                "User config incomplete.\n",
                "Please specify name and email.\n",
                "E.G.:\n",
                "mush config user.name 'James Smith'\n",
                "mush config user.email 'james@smith.com'"
            )
        ))
}
