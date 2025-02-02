
// Docs for git index format:
// https://github.com/git/git/blob/master/Documentation/gitformat-index.txt

use std::os::unix::fs::MetadataExt;

use crate::{cli::ContextlessCliResult, io::file_metadata};

// represents the staging area. Serialized into .mush/index
pub struct Index {
    entries: Vec<IndexEntry>
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
    hash: String, // [160]

    // (git) flags [16]
    assume_valid: bool,
    // TODO merge stage
    name_length: u16, // [12], min(0xFFF, object_name.len())

    file_name: String, // [null-terminated string]
}

impl Index {
    pub fn serialize(&self) -> Vec<u8> {
        [
            &b"DIRC"[..], // signature, stands for "dircache"
            &2u32.to_be_bytes(), // version 2
            &(self.entries.len() as u32).to_be_bytes(),
            self.entries.iter()
                .flat_map(|entry| entry.serialize())
                .collect::<Vec<_>>()
                .as_slice()
        ].concat()
    }

    pub fn new() -> Self {
        Index {
            entries: vec![],
        }
    }

    pub fn entries(&mut self) -> &mut Vec<IndexEntry> {
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
            &self.uid.to_be_bytes(),
            &self.gid.to_be_bytes(),
            &(self.size as u32).to_be_bytes(),
            self.hash.as_bytes(), &b"\0"[..],
            &(
                self.name_length.min(0xFFF) |
                ((if self.assume_valid { 1 } else { 0 }) << 15)
                // TODO merge stage?
            ).to_be_bytes(),
            self.file_name.as_bytes(), &b"\0"[..],
        ].concat()
    }

    pub fn new(file_name: &str, hash: &str) -> ContextlessCliResult<Self> {
        let stat = file_metadata(file_name)?;

        // TODO validate hash
        let hash = hash;
        // TODO canonicalize filename (relative to repo base, no leading dot or slash)
        let canonical_name = file_name;

        Ok(IndexEntry {
            metadata_change_time: (stat.ctime(), stat.ctime_nsec()),
            data_change_time: (stat.mtime(), stat.mtime_nsec()),
            device: stat.dev(),
            inode: stat.ino(),
            mode: stat.mode(),
            uid: stat.uid(),
            gid: stat.gid(),
            size: stat.size(),
            hash: String::from(hash),
            assume_valid: false,
            name_length: canonical_name.len() as u16,
            file_name: String::from(canonical_name),
        })
    }
}
