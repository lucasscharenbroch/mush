mod pretty_print;

use crate::{cli::CliResult, hash::Hash};

use std::borrow::Cow;

const COMPRESSION_LEVEL: u8 = 1;

pub enum ObjectType {
    Blob,
}

impl ObjectType {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Blob => "blob",
        }
    }

    fn from_string(string: &str) -> CliResult<Self> {
        match string {
            "blob" => Ok(Self::Blob),
            _ => Err(format!("Bad object type: `{string}`")),
        }
    }
}

pub enum Object<'b> {
    Blob(Cow<'b, [u8]>), // `Cow<'b, [u8]>` allows both owned ([u8]) and borrowed (&'b [u8])
                         // under the same interface
}

impl<'b> Object<'b> {
    pub fn store(&self) -> Vec<u8> {
        match self {
            Self::Blob(bytes) => {
                let header = format!("blob {}", bytes.len());
                [header.as_bytes(), &[b'\0'], bytes].concat()
            }
        }
    }

    fn unstore(mut bytes: Vec<u8>) -> CliResult<Self> {
        fn decode_contents<'b>(tipe: ObjectType, contents: Vec<u8>) -> CliResult<Object<'b>> {
            match tipe {
                ObjectType::Blob => Ok(Object::Blob(Cow::Owned(contents))),
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
        Hash::digest(self.store())
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
