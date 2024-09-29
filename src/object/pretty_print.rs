use super::Object;

impl<'b> std::fmt::Display for Object<'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blob(bytes) => {
                f.write_str(&String::from_utf8_lossy(bytes))
            }
        }
    }
}
