
// Docs for git index format:
// https://github.com/git/git/blob/master/Documentation/gitformat-index.txt

use std::{collections::BTreeMap, os::unix::fs::MetadataExt};

use crate::hash::Hash;
use crate::object::TreeEntry;

/// String newtype wrapper for a filename relative to the repo's base, no leading slash
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct RepoRelativeFilename(pub String);

impl std::ops::Deref for RepoRelativeFilename {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for RepoRelativeFilename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}

// represents the staging area. Serialized into .mush/index
pub struct Index {
    entries: BTreeMap<RepoRelativeFilename, IndexEntry>
}

pub struct IndexEntry {
    // based on [MetadataExt](https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html) types
    // bit sizes in serialized representation are in brackets

    metadata_change_time: (i64, i64), // ctime, ctime_nsec [32, 32]
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

    pub fn deserialize(bytes: &impl AsRef<[u8]>) -> Result<Self, String> {
        let bytes = bytes.as_ref();

        const MIN_POSSIBLE_BYTE_LENGTH: usize = 34;

        if bytes.len() < MIN_POSSIBLE_BYTE_LENGTH {
            return Err(String::from("Malformed index: header too small"));
        }

        let dirc = &bytes[..4];
        let version = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let num_entries = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let checksum = Hash::from_bytes(bytes[bytes.len() - 20..].try_into().unwrap());
        let checksum_input = &bytes[..bytes.len() - 20];
        let mut entry_list_bytes = bytes[12..bytes.len() - 22].iter().map(|b| *b).peekable();

        if dirc != b"DIRC" {
            return Err(format!("Malformed index: bad signature: {:?}", dirc));
        }

        if version != 2 {
            return Err(format!("Malformed index: bad version: {version} (expected 2)"));
        }

        if Hash::digest(&checksum_input.to_vec()) != checksum {
            return Err(format!("Malformed index: failed checksum"));
        }

        let mut entries = BTreeMap::new();

        while entry_list_bytes.peek().is_some() {
            let entry = IndexEntry::deserialize(&mut entry_list_bytes)?;
            entries.insert(entry.file_name.clone(), entry);
        }

        if entries.len() != num_entries as usize {
            return Err(format!("Malformed index: failed to parse expected number of entries"));
        }

        Ok(Index {
            entries
        })
    }

    pub fn empty() -> Self {
        Index {
            entries: BTreeMap::new(),
        }
    }

    pub fn entries(&mut self) -> &mut BTreeMap<RepoRelativeFilename, IndexEntry> {
        &mut self.entries
    }

    pub fn into_entries(self) -> BTreeMap<RepoRelativeFilename, IndexEntry> {
        self.entries
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
            // git adds extra null bytes to pad this to a multiple of 8 bytes.
            // we won't do that.
            self.file_name.as_bytes(), &b"\0"[..],
        ].concat()
    }

    pub fn deserialize(bytes: &mut impl Iterator<Item = u8>) -> Result<Self, String> {
        const FIXED_FIELDS_BYTE_SIZE: usize = 62; // all fields except the file name (which is variable-length)

        let header = bytes.take(FIXED_FIELDS_BYTE_SIZE).collect::<Vec<_>>();

        if header.len() < FIXED_FIELDS_BYTE_SIZE {
            return Err(String::from("Malformed index entry: too small"))
        }

        let ctime = u32::from_be_bytes(header[0..4].try_into().unwrap());
        let ctime_nsec = u32::from_be_bytes(header[4..8].try_into().unwrap());
        let mtime = u32::from_be_bytes(header[8..12].try_into().unwrap());
        let mtime_nsec = u32::from_be_bytes(header[12..16].try_into().unwrap());
        let dev = u32::from_be_bytes(header[16..20].try_into().unwrap());
        let ino = u32::from_be_bytes(header[20..24].try_into().unwrap());
        let mode = u32::from_be_bytes(header[24..28].try_into().unwrap());
        let uid = u32::from_be_bytes(header[28..32].try_into().unwrap());
        let gid = u32::from_be_bytes(header[32..36].try_into().unwrap());
        let size = u32::from_be_bytes(header[36..40].try_into().unwrap());
        let hash = Hash::from_bytes(header[40..60].try_into().unwrap());
        let flags = u16::from_be_bytes(header[60..62].try_into().unwrap());
        let filename_length = flags & 0xFFF;

        let filename_bytes = if filename_length < 0xFFF {
            // filename_length should be correct. No need to linear scan for null-byte.
            let res = bytes.take(filename_length as usize).collect::<Vec<_>>();

            if bytes.next() != Some(b'\0') {
                return Err(String::from("Incorrect filename length flag in index entry"));
            }

            res
        } else {
            // the take_while should consume the null byte
            bytes.take_while(|b| *b != b'\0')
                .collect::<Vec<_>>()
        };

        let file_name = String::from_utf8(filename_bytes)
            .map_err(|_err| String::from("Non-utf8 filename"))?;

        Ok(IndexEntry {
            metadata_change_time: (ctime as i64, ctime_nsec as i64),
            data_change_time: (mtime as i64, mtime_nsec as i64),
            device: dev as u64,
            inode: ino as u64,
            mode,
            uid,
            gid,
            size: size as u64,
            hash,

            assume_valid: flags & 0x8000 != 0,
            name_length: filename_length,
            file_name: RepoRelativeFilename(file_name),
        })
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

impl Into<TreeEntry> for IndexEntry {
    fn into(self) -> TreeEntry {
        TreeEntry::new(self.file_name, self.mode, self.hash)
    }
}
