use crate::hash::Hash;

/// A hypothetical pointer to an object (yet to be "dereferenced")
/// (could be a hash, ref, or expression involving the two)
pub enum RevisionSpec {
    // Many strings are ambiguous and could be either hashes or refs;
    // we can't know until checking the database, which happens after
    // this struct is instantiated.
    HashOrRef(String),
    NthParent(Box<RevisionSpec>, usize), // <rev>^[<n>]
    NthGenerationalParent(Box<RevisionSpec>, usize), // <rev>~[<n>]
}

impl RevisionSpec {
    fn parse(input: &str) -> Result<Self, String> {
        todo!()
    }

    /// Attempt to locate this revision in the database;
    /// return the hash or report why it couldn't be found
    fn dereference(&self) -> Result<Hash, String> {
        todo!()
    }
}
