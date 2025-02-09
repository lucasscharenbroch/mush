use crate::cli::CliResult;

pub struct User {
    pub name: String,
    pub email: String,
}

struct MushConfig {
    user: Option<User>,
}

pub fn force_get_user() -> CliResult<User> {
    // TODO
    Ok(User {
        name: String::from("James Smith"),
        email: String::from("james@smith.com"),
    })
}
