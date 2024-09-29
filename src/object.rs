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
}

/// Metadata contained in the object's header (type and size)
pub struct ObjectHeader {
    pub tipe: ObjectType,
    /// Size of the stored object, in bytes
    pub size: usize,
}

impl ObjectHeader {
    pub fn extract_from_file(file: impl std::io::Read, object_hash: &str) -> Result<Self, String> {
        // (`object_hash` is for diagnostic only)
        // ideally there would be no io logic in this module, but
        // efficient reading of the header (and not the rest of the object)
        // involves both io and format-related logic, and it goes here
        // because of the latter

        std::io::Read::bytes(flate2::read::DeflateDecoder::new(file))
            // make the Result clonable
            .map(|res| res.map_err(|io_err| io_err.to_string()))
            // take bytes until the null byte is encountered, collect errors
            .take_while(|res| res.clone().map(|b| b != b'\0').unwrap_or(true))
            .collect::<Result<Vec<_>, _>>() // (sequence/mapM) (fail on the first err)
            .map_err(|io_err| format!("Failed to read object `{object_hash}`: {io_err}"))
            .and_then(|bytes| {
                ObjectHeader::from_string(&String::from_utf8_lossy(bytes.as_slice()))
                    .map(|oh| Ok(oh))
                    .unwrap_or(Err(format!("Bad header of object `{object_hash}`")))
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
