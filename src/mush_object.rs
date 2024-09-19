use crate::hash::Hash;

enum MushObject<'b> {
    Blob(&'b [u8]),
}

impl<'b> MushObject<'b> {
    fn hash(&self) -> Hash {
        match self {
            Self::Blob(bytes) => {
                let header = format!("blob #{}\0", bytes.len());
                Hash::digest([header.as_bytes(), bytes].concat())
            },
            _ => todo!(),
        }
    }
}
