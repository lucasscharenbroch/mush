use sha1::{digest::generic_array::GenericArray, Digest};

#[derive(PartialEq)]
pub struct Hash {
    bytes: [u8; 20],
    string: String,
}

impl Hash {
    pub fn digest(data: &impl AsRef<[u8]>) -> Self {
        let bytes: GenericArray<u8, generic_array::typenum::U20> = sha1::Sha1::digest(data);
        Hash {
            bytes: bytes.into(),
            string: hex::encode(bytes),
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        hex::decode(string).ok()
            .and_then(|byte_vec| {
                byte_vec.try_into()
                    .ok()
                    .map(|bytes| {
                        Hash {
                            bytes,
                            string: String::from(string),
                        }
                    })
            })
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn path(&self) -> String {
        if self.len() < 2 {
            // obviously not a correct hash, but just return something plausible
            format!("objects/{}", self.as_str())
        } else {
            let (prefix, suffix) = self.split_at(2);
            format!("objects/{prefix}/{suffix}")
        }
    }
}

impl std::ops::Deref for Hash {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
