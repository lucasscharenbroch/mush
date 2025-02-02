use crate::cli::{CliResult, ContextlessCliResult};

pub fn create_directory_no_overwrite(directory: &str) -> ContextlessCliResult<()> {
    match std::fs::create_dir(directory) {
        Err(io_err) => {
            let directory = String::from(directory);
            if let std::io::ErrorKind::AlreadyExists = io_err.kind() {
                Err(Box::new(move |context| format!("Cannot {}: directory `{}` already exists", context, directory)))
            } else {
                Err(Box::new(move |context| format!("Failed to {}: error while creating directory `{}`: {}", context, directory, io_err)))
            }
        },
        _ => Ok(()),
    }
}

pub fn create_directories_no_overwrite<'a> (directories: impl Iterator<Item = &'a&'a str>) -> ContextlessCliResult<()> {
    directories
        .map(|dir| create_directory_no_overwrite(dir))
        .collect::<ContextlessCliResult<Vec<()>>>()
        .map(|_| ())
}

pub fn create_directory_all(directory: &str) -> ContextlessCliResult<()> {
    match std::fs::create_dir_all(directory) {
        Err(io_err) => {
            let directory = String::from(directory);
            if let std::io::ErrorKind::AlreadyExists = io_err.kind() {
                Err(Box::new(move |reason| format!("Cannot {}: directory `{}` already exists", reason, directory)))
            } else {
                Err(Box::new(move |reason| format!("Failed to {}: error while creating directory `{}`: {}", reason, directory, io_err)))
            }
        },
        _ => Ok(()),
    }
}

pub fn create_file_no_overwrite(filename: &str, contents: &[u8]) -> ContextlessCliResult<()> {
    match std::fs::File::create_new(filename) {
        Err(io_err) if matches!(io_err.kind(), std::io::ErrorKind::AlreadyExists) => {
            let filename = String::from(filename);
            return Err(Box::new(move |reason| format!("Cannot {}: file `{}` already exists", reason, filename)));
        },
        x => x,
    }.and_then(|mut file| {
        std::io::Write::write_all(&mut file, contents)
    })
    .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err| {
        let filename = String::from(filename);
        Box::new(move |reason: &str|
            format!("Failed to {}: error while creating file `{}`: {}", reason, filename, io_err)
        )
    })
}

pub fn create_file_all_no_overwrite(filename: &str, contents: &[u8]) -> ContextlessCliResult<()> {
    let path = std::path::Path::new(filename);
    let directory = path.parent().unwrap_or(std::path::Path::new("."));

    create_directory_all(directory.to_str().unwrap())?;
    create_file_no_overwrite(path.to_str().unwrap(), contents)?;
    Ok(())
}

pub fn create_file(filename: &str, contents: &[u8]) -> ContextlessCliResult<()> {
    std::fs::File::create(filename)
    .and_then(|mut file| {
        std::io::Write::write_all(&mut file, contents)
    })
    .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err| {
        let filename = String::from(filename);
        Box::new(move |reason: &str|
            format!("Failed to {}: error while creating file `{}`: {}", reason, filename, io_err)
        )
    })
}

pub fn create_file_all(filename: &str, contents: &[u8]) -> ContextlessCliResult<()> {
    let path = std::path::Path::new(filename);
    let directory = path.parent().unwrap_or(std::path::Path::new("."));

    create_directory_all(directory.to_str().unwrap())?;
    create_file(path.to_str().unwrap(), contents)?;
    Ok(())
}

pub fn read_file_to_str(file: std::fs::File, filename: &str) -> ContextlessCliResult<String> {
    let filename = String::from(filename);
    std::io::read_to_string(file)
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(
            |io_err|
            Box::new(move |reason| format!("Failed to {}: error while reading file `{}`: {}", reason, filename, io_err))
        )
}

pub fn read_file_to_bytes(file: &mut std::fs::File, filename: &str) -> ContextlessCliResult<Vec<u8>> {
    let filename = String::from(filename);
    let mut res = Vec::new();
    std::io::Read::read_to_end(file, &mut res)
        .map(|_| res)
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
            Box::new(move |reason| format!("Failed to {}: error while reading file `{}`: {}", reason, filename, io_err))
        )
}

pub fn open_filename(filename: &str) -> ContextlessCliResult<std::fs::File> {
    let filename = String::from(filename);
    std::fs::File::open(&filename)
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
            Box::new(move |reason| format!("Failed to {}: error while opening file `{}`: {}", reason, filename, io_err))
        )
}

pub fn read_filename_to_str(filename: &str) -> ContextlessCliResult<String> {
    read_file_to_str(open_filename(filename)?, filename)
}

pub fn read_filename_to_bytes(filename: &str) -> ContextlessCliResult<Vec<u8>> {
    read_file_to_bytes(&mut open_filename(filename)?, filename)
}

pub fn read_filename_or_stdin_to_str(filename: &str) -> ContextlessCliResult<String> {
    if filename == "-" { // stdin
        let filename = String::from(filename);
        std::io::read_to_string(std::io::stdin())
            .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
                Box::new(move |reason| format!("Failed to {}: error while reading stdin `{}`: {}", reason, filename, io_err))
            )
    } else { // normal filename
        read_filename_to_str(filename)
    }
}

pub fn file_metadata(filename: &str) -> ContextlessCliResult<std::fs::Metadata> {
    let filename = String::from(filename);
    std::fs::metadata(&filename)
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
            Box::new(move |reason| format!("Failed to {}: error while fetching metadata of file `{}`: {}", reason, filename, io_err))
        )
}

pub fn dot_mush_folder() -> ContextlessCliResult<String> {
    const DOT_MUSH: &'static str = ".mush";

    let mut dir = std::env::current_dir()
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
            Box::new(move |reason| format!("Failed to {}: error while getting cwd: {}", reason, io_err))
        )?;

    loop {
        let target = dir.join(DOT_MUSH);
        if target.exists() {
            return target.to_str()
                .map(|str| String::from(str))
                .ok_or::<Box<dyn FnOnce(&str) -> String>>(
                    Box::new(move |reason| format!("Failed to {}: error while getting cwd: failed to convert path to string", reason))
                );
        }

        if let Some(parent) = dir.parent() {
                dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    Err(Box::new(move |reason| format!("Failed to {}: not a mush repository", reason)))
}

pub fn dot_mush_slash(path: &str) -> ContextlessCliResult<String> {
    Ok(format!("{}/{}", dot_mush_folder()?, path))
}
