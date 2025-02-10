use itertools::Itertools;

use crate::{cli::{with_context, CliResult, ContextlessCliResult}, io::{dot_mush_slash, read_filename_to_str, try_read_filename_to_str}};

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

impl MushConfig {
    fn _read() -> ContextlessCliResult<MushConfig> {
        //< Use a filesystem-based, per-repository (non-global) config.
        //< Writing a global config would be nicer for actual use,
        //< but it would be a hairier (though not much more interesting)
        //< implementation, plus making testing much more tedious.

        fn read_field(field_name: &str) -> ContextlessCliResult<Option<String>> {
            let target_file = dot_mush_slash(
                &format!("config/{}", field_name.split(".").join("/"))
            )?;

            try_read_filename_to_str(&target_file)
        }

        Ok(
            MushConfig {
                user: PartialUser {
                    name: read_field("user.name")?,
                    email: read_field("user.email")?,
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
        .ok_or(String::from("User config incomplete. Please specify name and email."))
}
