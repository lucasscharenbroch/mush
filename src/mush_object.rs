use crate::hash::Hash;

const COMPRESSION_LEVEL: u8 = 6;

pub enum MushObject<'b> {
    Blob(&'b [u8]),
}

impl<'b> MushObject<'b> {
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
