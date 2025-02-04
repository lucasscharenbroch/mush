use super::Object;

impl<'b> std::fmt::Display for Object<'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blob(bytes) => f.write_str(&String::from_utf8_lossy(bytes)),
            Self::Tree(entries) => {
                for entry in entries.iter() {
                    f.write_str(&format!(
                        "{:06} {} {}\t{}\n",
                        entry.mode,
                        entry.object_type.to_str(),
                        entry.hash.as_str(),
                        entry.filename,
                    ))?;
                }

                Ok(())
            }
        }
    }
}
