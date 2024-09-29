use sha1::{digest::generic_array::GenericArray, Digest};

/// Newtype (wrapper) for `String`
#[derive(PartialEq)]
pub enum Hash {
    Hash(String),
}

impl Hash {
    pub fn digest(data: impl AsRef<[u8]>) -> Self {
        let hash_as_bytes: GenericArray<u8, generic_array::typenum::U20> = sha1::Sha1::digest(data);
        Self::Hash(hex::encode(hash_as_bytes))
    }

    pub fn as_str(&self) -> &str {
        &self
    }

    pub fn path(&self) -> String {
        let (prefix, suffix) = self.split_at(2);
        format!("objects/{}/{}", prefix, suffix)
    }
}

impl std::ops::Deref for Hash {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hash(s) => s,
        }
    }
}
