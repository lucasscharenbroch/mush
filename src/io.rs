#[macro_export]
macro_rules! create_directories_no_overwrite {
    ($dirs:expr, $reason:expr) => {
        for dir in $dirs {
            crate::create_directory_no_overwrite!(dir, $reason);
        }
    };
}

#[macro_export]
macro_rules! create_directory_no_overwrite {
    ($dir:expr, $reason:expr) => {
        match std::fs::create_dir($dir) {
            Err(io_err) => {
                if let std::io::ErrorKind::AlreadyExists = io_err.kind() {
                    eprintln!("Cannot {}: directory `{}` already exists", $reason, $dir);
                } else {
                    eprintln!("Failed to {}: error while creating directory `{}`: {}", $reason, $dir, io_err);
                }

                return crate::cli::ExitType::Fatal;
            },
            _ => (),
        }
    };
}

#[macro_export]
macro_rules! create_directory_all {
    ($dir:expr, $reason:expr) => {
        match std::fs::create_dir_all($dir) {
            Err(io_err) => {
                if let std::io::ErrorKind::AlreadyExists = io_err.kind() {
                    eprintln!("Cannot {}: directory `{}` already exists", $reason, $dir);
                } else {
                    eprintln!("Failed to {}: error while creating directory `{}`: {}", $reason, $dir, io_err);
                }

                return crate::cli::ExitType::Fatal;
            },
            _ => (),
        }
    };
}

#[macro_export]
macro_rules! create_file_no_overwrite {
    ($file:expr, $contents:expr, $reason:expr) => {
        let res = match std::fs::File::create($file) {
            Err(io_err) if matches!(io_err.kind(), std::io::ErrorKind::AlreadyExists) => {
                eprintln!("Cannot {}: file `{}` already exists", $reason, $file);
                return crate::cli::ExitType::Fatal;
            },
            x => x,
        }.and_then(|mut file| {
            std::io::Write::write_all(&mut file, $contents)
        });

        if let Err(io_err) = res {
            eprintln!("Failed to {}: error while creating file `{}`: {}", $reason, $file, io_err);
            return crate::cli::ExitType::Fatal;
        }
    };
}

#[macro_export]
macro_rules! create_file_all_no_overwrite {
    ($file:expr, $contents:expr, $reason:expr) => {
        let path = std::path::Path::new($file);
        let directory = path.parent().unwrap_or(std::path::Path::new("."));

        crate::create_directory_all!(directory.to_str().unwrap(), $reason);
        crate::create_file_no_overwrite!(path.to_str().unwrap(), $contents, $reason);
    }
}

#[macro_export]
macro_rules! read_file_or_stdin {
    ($filename:expr, $reason:expr) => {
        if $filename == "-" { // stdin
            match std::io::read_to_string(std::io::stdin()) {
                Ok(string) => string,
                Err(io_err) => {
                    eprintln!("Failed to {}: error while reading stdin: {}", $reason, io_err);
                    return crate::cli::ExitType::Fatal;
                }
            }
        } else { // normal filename
            let file = match std::fs::File::open(&$filename) {
                Ok(file) => file,
                Err(io_err) => {
                    eprintln!("Failed to {}: error while reading file `{}`: {}", $reason, $filename, io_err);
                    return crate::cli::ExitType::Fatal;
                }
            };

            match std::io::read_to_string(file) {
                Ok(string) => string,
                Err(io_err) => {
                    eprintln!("Failed to {}: error while reading file `{}`: {}", $reason, $filename, io_err);
                    return crate::cli::ExitType::Fatal;
                }
            }
        }
    };
}

#[macro_export]
macro_rules! dot_mush_slash {
    ($path:expr) => {
        // TODO traverse upward and try to find `.mush`;
        // assert that cwd is in a workspace
        // (not directly within `.mush`, but a child of a directory adjacent to it)
        format!(".mush/{}", $path)
    };
}
