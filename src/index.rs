
// Docs for git index format:
// https://github.com/git/git/blob/master/Documentation/gitformat-index.txt

// represents the staging area. Serialized into .mush/index
struct Index {
    entries: Vec<IndexEntry>
}

struct IndexEntry {
    // based on [MetadataExt](https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html) types
    // bit sizes in serialized representation are in brackets

    metadata_change_time: (i64, i64), // ctime, mtime_nsec [32, 32]
    data_change_time: (i64, i64), // mtime, mtime_nsec [32, 32]
    device: u64, // dev [32]
    inode: u64, // ino [32]
    mode: u32, // ino [32]
    uid: u32, // uid [32]
    gid: u32, // gid [32]
    size: u64, // size [32]
    hash: String, // [160]

    // (git) flags [16]
    assume_valid: bool,
    // TODO merge stage
    name_length: u16, // [12], min(0xFFF, object_name.len())

    object_name: String, // [null-terminated string]
}

impl Index {
    fn serialize(&self) -> Vec<u8> {
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
            self.object_name.as_bytes(), &b"\0"[..],
        ].concat()
    }
}
