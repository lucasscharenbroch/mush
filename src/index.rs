
// Docs for git index format:
// https://github.com/git/git/blob/master/Documentation/gitformat-index.txt

use std::{collections::BTreeMap, os::unix::fs::MetadataExt};

use crate::hash::Hash;

/// String newtype wrapper for a filename relative to the repo's base, no leading slash
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct RepoRelativeFilename(String);

impl std::ops::Deref for RepoRelativeFilename {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// represents the staging area. Serialized into .mush/index
pub struct Index {
    entries: BTreeMap<RepoRelativeFilename, IndexEntry>
}

pub struct IndexEntry {
    // based on [MetadataExt](https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html) types
    // bit sizes in serialized representation are in brackets

    metadata_change_time: (i64, i64), // ctime, mtime_nsec [32, 32]
    data_change_time: (i64, i64), // mtime, mtime_nsec [32, 32]
    device: u64, // dev [32]
    inode: u64, // ino [32]
    mode: u32, // mode [32]
    uid: u32, // uid [32]
    gid: u32, // gid [32]
    size: u64, // size [32]
    hash: Hash, // [160]

    // (git) flags [16]
    assume_valid: bool,
    // TODO merge stage
    name_length: u16, // [12], min(0xFFF, object_name.len())

    file_name: RepoRelativeFilename, // [null-terminated string]
}

impl Index {
    pub fn serialize(&self) -> Vec<u8> {
        let mut byte_content = [
            &b"DIRC"[..], // signature, stands for "dircache"
            &2u32.to_be_bytes(), // version 2
            &(self.entries.len() as u32).to_be_bytes(),
            self.entries.iter()
                .flat_map(|(_filename, entry)| entry.serialize())
                .collect::<Vec<_>>()
                .as_slice(),
            &0u16.to_be_bytes(), // size of extension
        ].concat();

        let checksum = Hash::digest(&byte_content);
        byte_content.extend(checksum.as_bytes());

        byte_content
    }

    pub fn new() -> Self {
        Index {
            entries: BTreeMap::new(),
        }
    }

    pub fn entries(&mut self) -> &mut BTreeMap<RepoRelativeFilename, IndexEntry> {
        &mut self.entries
    }
}

impl IndexEntry {
    fn serialize(&self) -> Vec<u8> {
        [
            &(self.metadata_change_time.0 as u32).to_be_bytes(),
            &(self.metadata_change_time.1 as u32).to_be_bytes(),
            &(self.data_change_time.0 as u32).to_be_bytes(),
            &(self.data_change_time.1 as u32).to_be_bytes(),
            &(self.device as u32).to_be_bytes(),
            &(self.inode as u32).to_be_bytes(),
            &self.mode.to_be_bytes(),
            &self.uid.to_be_bytes(),
            &self.gid.to_be_bytes(),
            &(self.size as u32).to_be_bytes(),
            self.hash.as_bytes(),
            &(
                self.name_length.min(0xFFF) |
                ((if self.assume_valid { 1 } else { 0 }) << 15)
                // TODO merge stage?
            ).to_be_bytes(),
            self.file_name.as_bytes(), &b"\0"[..],
        ].concat()
    }

    pub fn new(filename: RepoRelativeFilename, hash: Hash, metadata: std::fs::Metadata) -> Self {
        IndexEntry {
            metadata_change_time: (metadata.ctime(), metadata.ctime_nsec()),
            data_change_time: (metadata.mtime(), metadata.mtime_nsec()),
            device: metadata.dev(),
            inode: metadata.ino(),
            mode: metadata.mode(),
            uid: metadata.uid(),
            gid: metadata.gid(),
            size: metadata.size(),
            hash,
            assume_valid: false,
            name_length: filename.len() as u16,
            file_name: filename,
        }
    }
}

// this conceptually belongs in the io module, but put here to keep
// construction of `RepoRelativeFilename` private to this module
/// Convert a filename to its canonical representation in the index
/// (relative to the mush repository, without any leading slash)
pub fn repo_canononicalize(filename: &str) -> crate::cli::ContextlessCliResult<RepoRelativeFilename> {
    let filename = String::from(filename);
    let repo_directory = crate::io::repo_folder()?;
    let repo_directory = crate::io::canonicalize(&repo_directory)?;

    let canonical_filename = crate::io::canonicalize(&filename)?;

    eprintln!("a: {canonical_filename:?} b: {repo_directory:?}");

    canonical_filename
        .strip_prefix(&repo_directory)
        .map_err(|err| format!("{err}"))
        .and_then(|path| path.to_str().ok_or(String::from("Failed to convert path to string")))
        .map(|path_str| RepoRelativeFilename(String::from(path_str)))
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(
            |err_str|
            Box::new(move |reason| format!("Failed to {}: error while reading file `{}`: {}", reason, filename, err_str))
        )

    /*
    std::path::Path::new(&filename).canonicalize()
        .map_err(|io_err| io_err.to_string())
        .and_then(|file_path|
            file_path.strip_prefix(&mush_directory)
                .map_err(|err| format!("{err}"))
                .and_then(|path| path.to_str().ok_or(String::from("Failed to convert path to string")))
                .map(|path_str| String::from(path_str))
        )
        .map_err::<Box<dyn FnOnce(&str) -> String>, _>(
            |io_err|
            Box::new(move |reason| format!("Failed to {}: error while reading file `{}`: {}", reason, filename, io_err))
        )
        .map(|str| RepoRelativeFilename(String::from(str)))
    */
}
