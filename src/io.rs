use std::fs::ReadDir;

use crate::cli::{with_context, CliResult, ContextlessCliResult};
use crate::index::{Index, RepoRelativeFilename};
use crate::object::{Object, ObjectHeader};
use crate::hash::Hash;

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

pub fn create_directory_all_idempotent(directory: &str) -> ContextlessCliResult<()> {
    match std::fs::create_dir_all(directory) {
        Err(io_err) => {
            if let std::io::ErrorKind::AlreadyExists = io_err.kind() {
                Ok(())
            } else {
                let directory = String::from(directory);
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

    create_directory_all_idempotent(directory.to_str().unwrap())?;
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

pub fn overwrite_file(filename: &str, contents: &[u8]) -> ContextlessCliResult<()> {
    std::fs::OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(filename)
        .and_then(|mut file| {
            std::io::Write::write_all(&mut file, contents)
        })
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err| {
            let filename = String::from(filename);
            Box::new(move |reason: &str|
                format!("Failed to {}: error while overwriting file `{}`: {}", reason, filename, io_err)
            )
        })
}

pub fn create_file_all(filename: &str, contents: &[u8]) -> ContextlessCliResult<()> {
    let path = std::path::Path::new(filename);
    let directory = path.parent().unwrap_or(std::path::Path::new("."));

    create_directory_all_idempotent(directory.to_str().unwrap())?;
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

pub fn try_open_filename(filename: &str) -> ContextlessCliResult<Option<std::fs::File>> {
    let filename = String::from(filename);

    if !std::path::Path::new(&filename).exists() {
        return Ok(None);
    }

    std::fs::File::open(&filename)
        .map(|file| Some(file))
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
            Box::new(move |reason| format!("Failed to {}: error while opening file `{}`: {}", reason, filename, io_err))
        )
}

pub fn open_file(path: &std::path::Path) -> ContextlessCliResult<std::fs::File> {
    let filename = String::from(path.to_str().unwrap_or("<unprintable-path>"));
    std::fs::File::open(&path)
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
            Box::new(move |reason| format!("Failed to {}: error while opening file `{}`: {}", reason, filename, io_err))
        )
}

pub fn try_read_filename_to_str(filename: &str) -> ContextlessCliResult<Option<String>> {
    let file_opt = try_open_filename(filename)?;

    match file_opt {
        None => Ok(None),
        Some(file) => read_file_to_str(file, filename).map(|s| Some(s))
    }
}

pub fn read_filename_to_str(filename: &str) -> ContextlessCliResult<String> {
    read_file_to_str(open_filename(filename)?, filename)
}

pub fn read_filename_to_bytes(filename: &str) -> ContextlessCliResult<Vec<u8>> {
    read_file_to_bytes(&mut open_filename(filename)?, filename)
}

pub fn read_stdin_to_str() -> ContextlessCliResult<String> {
    std::io::read_to_string(std::io::stdin())
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
            Box::new(move |reason| format!("Failed to {}: error while reading stdin: {}", reason, io_err))
        )
}

pub fn read_filename_or_stdin_to_str(filename: &str) -> ContextlessCliResult<String> {
    if filename == "-" { // stdin
        std::io::read_to_string(std::io::stdin())
            .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
                Box::new(move |reason| format!("Failed to {}: error while reading stdin: {}", reason, io_err))
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

pub fn file_exists(filename: &str) -> bool {
    std::path::Path::new(&filename).exists()
}

pub fn repo_folder() -> ContextlessCliResult<String> {
    const DOT_MUSH: &'static str = ".mush";

    let mut dir = std::env::current_dir()
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
            Box::new(move |reason| format!("Failed to {}: error while getting cwd: {}", reason, io_err))
        )?;

    loop {
        let target = dir.join(DOT_MUSH);
        if target.exists() {
            return dir.to_str()
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

pub fn dot_mush_folder() -> ContextlessCliResult<String> {
    repo_folder()
        .map(|path| format!("{path}/.mush"))
}

pub fn dot_mush_slash(path: &str) -> ContextlessCliResult<String> {
    Ok(format!("{}/{}", dot_mush_folder()?, path))
}

pub fn canonicalize_without_forcing_existance(path_str: &str) -> ContextlessCliResult<std::path::PathBuf> {
    let path = std::path::Path::new(path_str);
    if path.exists() {
        canonicalize(path_str)
    } else if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .map_err::<Box<dyn FnOnce(&str) -> String>, _>(|io_err|
                Box::new(move |reason| format!("Failed to {}: error while getting cwd: {}", reason, io_err))
            )
    }
}

pub fn canonicalize(path: &str) -> ContextlessCliResult<std::path::PathBuf> {
    let path = String::from(path);
    std::path::Path::new(&path)
        .canonicalize()
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(
            |io_err|
            Box::new(move |reason| format!("Failed to {}: error while canonicalizing filename `{}`: {}", reason, path, io_err))
        )
}

// Parse .mush/index, if it exists
// Ok(None) means it doesn't exist
pub fn read_index() -> ContextlessCliResult<Option<Index>> {
    let index_filename = dot_mush_slash("index")?;
    if !std::path::Path::new(&index_filename).exists() {
        Ok(None)
    } else {
        let bytes = read_filename_to_bytes(&index_filename)?;

        Index::deserialize(&bytes)
            .map(|index| Some(index))
            .map_err::<Box<dyn FnOnce(&str) -> String>, _>( |err_str|
                Box::new(move |reason| format!("Failed to {}: error while reading .mush/index: {}", reason, err_str))
            )
    }
}

/// Convert a filename to its canonical representation in the index
/// (relative to the mush repository, without any leading slash)
pub fn repo_canononicalize(filename: &str) -> crate::cli::ContextlessCliResult<RepoRelativeFilename> {
    let filename = String::from(filename);
    let repo_directory = crate::io::repo_folder()?;
    let repo_directory = crate::io::canonicalize(&repo_directory)?;

    let canonical_filename = crate::io::canonicalize_without_forcing_existance(&filename)?;

    canonical_filename
        .strip_prefix(&repo_directory)
        .map_err(|err| format!("{err}"))
        .and_then(|path| path.to_str().ok_or(String::from("Failed to convert path to string")))
        .map(|path_str| RepoRelativeFilename(String::from(path_str)))
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(
            |err_str|
            Box::new(move |reason| format!("Failed to {}: error while reading file `{}`: {}", reason, filename, err_str))
        )
}

pub fn write_object(object: &Object) -> CliResult<()> {
    let target_file = with_context("resole path", dot_mush_slash(&object.hash().path()))?;
    with_context("write object", create_file_all(&target_file, object.compressed().as_slice()))
}

pub fn read_object_header(hash: &Hash) -> CliResult<ObjectHeader> {
    let object_filename = with_context("resolve path", dot_mush_slash(&hash.path()))?;
    let file = with_context("get object header", open_filename(&object_filename))?;
    ObjectHeader::extract_from_file(file, &hash)
}

pub fn read_object(hash: &Hash) -> CliResult<Object<'static>> {
    let object_filename = with_context("resolve path", dot_mush_slash(&hash.path()))?;
    let object_contents_str = with_context("read object", read_filename_to_bytes(&object_filename))?;
    Object::from_compressed_bytes(
        &object_contents_str
    )
        .map_err(|msg| format!("Error while reading object: {msg}"))
}

pub fn cwd_iter() -> ContextlessCliResult<ReadDir> {
    std::fs::read_dir(".")
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(
            |io_err|
            Box::new(move |reason| format!("Failed to {reason}: error while reading cwd: {io_err}"))
        )
}
