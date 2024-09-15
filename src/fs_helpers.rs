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

                return crate::cli_helpers::ExitType::Fatal;
            },
            _ => (),
        }
    };
}

#[macro_export]
macro_rules! create_file_no_overwrite {
    ($file:expr, $contents:expr, $reason:expr) => {
        let _ = match std::fs::File::create($file) {
            Err(io_err) if matches!(io_err.kind(), std::io::ErrorKind::AlreadyExists) => {
                eprintln!("Cannot {}: file `{}` already exists", $reason, $file);
                return crate::cli_helpers::ExitType::Fatal;
            },
            x => x,
        }.and_then(|mut file| {
            std::io::Write::write_all(&mut file, $contents)
        }).map_err(|io_err| {
            eprintln!("Failed to {}: error while creating file `{}`: {}", $reason, $file, io_err);
            return crate::cli_helpers::ExitType::Fatal;
        });
    };
}
