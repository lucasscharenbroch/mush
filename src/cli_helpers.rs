use std::process::ExitCode;

pub enum ExitType {
    Ok,
    Fatal,
}

impl Into<ExitCode> for ExitType {
    fn into(self) -> ExitCode {
        match self {
            Self::Ok => ExitCode::SUCCESS,
            Self::Fatal => ExitCode::from(128),
        }
    }
}
