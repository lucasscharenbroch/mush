mod pretty_print;

use crate::{cli::{CliResult, ContextlessCliResult}, hash::Hash, index::RepoRelativeFilename};

use std::{borrow::Cow, collections::btree_map::Entry};

const COMPRESSION_LEVEL: u8 = 1;

pub enum ObjectType {
    Blob,
    Tree,
}

impl ObjectType {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Blob => "blob",
            Self::Tree => "tree",
        }
    }

    fn from_string(string: &str) -> CliResult<Self> {
        match string {
            "blob" => Ok(Self::Blob),
            "tree" => Ok(Self::Tree),
            _ => Err(format!("Bad object type: `{string}`")),
        }
    }
}

pub enum Object<'b> {
    Blob(Cow<'b, [u8]>), // `Cow<'b, [u8]>` allows both owned ([u8]) and borrowed (&'b [u8])
                         // under the same interface
    Tree(Vec<TreeEntry>),
}

pub struct TreeEntry {
    filename: RepoRelativeFilename,
    mode: u32,
    hash: Hash,
    object_type: ObjectType,
}

impl TreeEntry {
    pub fn store(&self) -> Vec<u8> {
        [
            format!("{:06}", self.mode).as_bytes(),
            b" ",
            self.filename.as_bytes(),
            b"\0",
            self.hash.as_bytes(),
        ].concat()
    }

    fn unstore(bytes: &mut impl Iterator<Item = u8>) -> CliResult<Self> {
        let mode = bytes.take(6).collect::<Vec<_>>();
        let space = bytes.take(1).collect::<Vec<_>>();
        let filename = bytes.take_while(|b| *b != b'\0').collect::<Vec<_>>();
        let hash = bytes.take(20).collect::<Vec<_>>();

        if mode.len() != 6 || space.len() != 1 || space[0] != b' ' || hash.len() != 20 {
            return Err(String::from("Malformed tree object"));
        }

        let object_type = ObjectType::Blob; // TODO

        Ok(TreeEntry {
            mode: String::from_utf8(mode)
                .map_err(|_| ())
                .and_then(|s| s.parse::<u32>().map_err(|_| ()))
                .map_err(|_| String::from("Malformed tree object: bad mode string"))?,
            filename: RepoRelativeFilename(
                String::from_utf8(filename)
                    .map_err(|_| String::from("Malformed index: bad filename"))?
            ),
            hash: Hash::from_bytes(hash.as_slice().try_into().unwrap()),
            object_type,
        })
    }
}

impl<'b> Object<'b> {
    pub fn store(&self) -> Vec<u8> {
        match self {
            Self::Blob(bytes) => {
                let header = format!("blob {}", bytes.len());
                [header.as_bytes(), &[b'\0'], bytes].concat()
            },
            Self::Tree(entries) => {
                let net_entry_byte_size = entries.iter()
                    .map(|entry|
                        6 + // mode
                        1 + // space
                        entry.filename.len() +
                        1 + // null-byte
                        20 // hash
                    )
                    .sum::<usize>();
                let header = format!("tree {}", net_entry_byte_size);

                [
                    header.as_bytes(),
                    &[b'\0'],
                    entries.iter().flat_map(|entry| entry.store())
                        .collect::<Vec<_>>()
                        .as_slice(),
                ].concat()
            }
        }
    }

    fn unstore(mut bytes: Vec<u8>) -> CliResult<Self> {
        fn decode_contents<'b>(tipe: ObjectType, contents: Vec<u8>) -> CliResult<Object<'b>> {
            match tipe {
                ObjectType::Blob => Ok(Object::Blob(Cow::Owned(contents))),
                ObjectType::Tree => {
                    let mut contents_iterator = contents.iter().map(|b| *b).peekable();
                    let mut entries = Vec::new();

                    while contents_iterator.peek().is_some() {
                        entries.push(TreeEntry::unstore(&mut contents_iterator)?);
                    }

                    Ok(Object::Tree(entries))
                }
            }
        }

        if let Some(null_byte_idx) = bytes.iter().position(|b| *b == b'\0') {
            let contents = bytes.split_off(null_byte_idx + 1);
            let header = ObjectHeader::from_bytes(&bytes[..null_byte_idx])?;

            if header.size != contents.len() {
                Err(String::from(
                    "Corrupt object (mismatched header and contents size)",
                ))
            } else {
                decode_contents(header.tipe, contents)
            }
        } else {
            Err(String::from("Invalid header (no null byte)"))
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::digest(&self.store())
    }

    pub fn compressed(&self) -> Vec<u8> {
        miniz_oxide::deflate::compress_to_vec_zlib(self.store().as_slice(), COMPRESSION_LEVEL)
    }

    pub fn from_compressed_bytes(bytes: &[u8]) -> CliResult<Object<'b>> {
        miniz_oxide::inflate::decompress_to_vec_zlib(bytes)
            .map_err(|err| err.to_string())
            .and_then(|decompressed_bytes| Self::unstore(decompressed_bytes))
    }
}

/// Metadata contained in the object's header (type and size)
pub struct ObjectHeader {
    pub tipe: ObjectType,
    /// Size of the stored object, in bytes
    pub size: usize,
}

impl ObjectHeader {
    pub fn extract_from_file(file: impl std::io::Read, object_hash: &str) -> CliResult<Self> {
        // (`object_hash` is for diagnostic only)
        // ideally there would be no IO logic in this module, but
        // efficient reading of the header (and not the rest of the object)
        // involves both io and format-related logic, and it goes here
        // because of the latter

        // TODO factor out with "read/decode prefix" macro? or the like?

        std::io::Read::bytes(flate2::read::ZlibDecoder::new(file))
            // make the Result clonable
            .map(|res| res.map_err(|io_err| io_err.to_string()))
            // take bytes until the null byte is encountered, collect errors
            .take_while(|res| res.clone().map(|b| b != b'\0').unwrap_or(true))
            .collect::<CliResult<Vec<_>>>() // (sequence/mapM) (fail on the first err)
            .and_then(|bytes| ObjectHeader::from_bytes(bytes.as_slice()))
            .map_err(|msg| format!("Failed to read object `{object_hash}`: {msg}"))
    }

    fn from_bytes(bytes: &[u8]) -> CliResult<Self> {
        // e.g. "blob 1234"
        //       ^^^^ ^^^^
        let string = String::from_utf8_lossy(bytes);
        let segments = string.split(" ").collect::<Vec<_>>();

        if segments.len() != 2 {
            Err(String::from("Bad object header"))
        } else {
            let (type_str, size_str) = (segments[0], segments[1]);
            ObjectType::from_string(type_str)
                .map_err(|msg| format!("Bad object header: {msg}"))
                .and_then(|tipe| {
                    size_str
                        .parse::<usize>()
                        .map_err(|err| err.to_string())
                        .map(|size| ObjectHeader { tipe, size })
                })
        }
    }
}
