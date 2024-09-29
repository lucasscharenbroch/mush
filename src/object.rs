mod pretty_print;

use crate::hash::Hash;

const COMPRESSION_LEVEL: u8 = 6;

pub enum ObjectType {
    Blob,
}

impl ObjectType {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Blob => "blob",
        }
    }

    fn from_string(string: &str) -> Option<Self> {
        match string {
            "blob" => Some(Self::Blob),
            _ => None
        }
    }
}

pub enum Object<'b> {
    Blob(&'b [u8]),
}

impl<'b> Object<'b> {
    pub fn store(&self) -> Vec<u8> {
        match self {
            Self::Blob(bytes) => {
                let header = format!("blob {}\0", bytes.len());
                [header.as_bytes(), bytes].concat()
            },
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::digest(self.store())
    }

    pub fn compressed(&self) -> Vec<u8> {
        miniz_oxide::deflate::compress_to_vec(self.store().as_slice(), COMPRESSION_LEVEL)
    }

    pub fn object_path(&self) -> String {
        let hash = self.hash();
        let (prefix, suffix) = hash.split_at(2);
        format!("objects/{}/{}", prefix, suffix)
    }
}

/// Metadata contained in the object's header (type and size)
pub struct ObjectHeader {
    tipe: ObjectType,
    /// Size of the stored object, in bytes
    size: usize,
}

impl ObjectHeader {
    fn extract_from_file(file: impl std::io::Read, object_hash: &str) -> Result<Self, String> {
        // (`object_hash` is for diagnostic only)

        file.bytes()
            .map(|res| res.map_err(|io_err| io_err.to_string()))
            // take bytes until the null byte is encountered, collect errors
            .take_while(|res| res.clone().map(|b| b != b'0').unwrap_or(true))
            .collect::<Result<Vec<_>, _>>() // (sequence/mapM)
            .map_err(|io_err| format!("Failed to read object `{object_hash}`: {io_err}"))
            .and_then(|bytes| {
                ObjectHeader::from_string(&String::from_utf8_lossy(bytes.as_slice()))
                    .map(|oh| Ok(oh))
                    .unwrap_or(Err(String::from("Bad object header (`{object_hash}`)")))
            })
    }

    fn from_string(string: &str) -> Option<Self> {
        // e.g. "blob 1234"
        //       ^^^^ ^^^^
        let segments = string.split(" ").collect::<Vec<_>>();

        if segments.len() != 2 {
            None
        } else {
            let (type_str, size_str) = (segments[0], segments[1]);
            ObjectType::from_string(type_str).and_then(|tipe| {
                size_str.parse::<usize>().ok().map(|size| {
                    ObjectHeader {
                        tipe,
                        size
                    }
                })
            })
        }
    }
}
