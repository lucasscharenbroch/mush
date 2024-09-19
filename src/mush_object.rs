use crate::hash::Hash;

pub enum MushObject<'b> {
    Blob(&'b [u8]),
}

impl<'b> MushObject<'b> {
    pub fn hash(&self) -> Hash {
        match self {
            Self::Blob(bytes) => {
                let header = format!("blob {}\0", bytes.len());
                Hash::digest([header.as_bytes(), bytes].concat())
            },
        }
    }
}
