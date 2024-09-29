use crate::hash::Hash;

/// A hypothetical pointer to an object (yet to be `dereference`d)
/// (could be a hash, ref, or expression involving the two)
pub struct RevisionSpec<'s> {
    original_input: &'s str,
    parse_tree: RevisionSpecParseTree
}

impl<'s> RevisionSpec<'s> {
    pub fn parse(input: &'s str) -> Result<Self, String> {
        RevisionSpecParseTree::parse(input).map(|parse_tree| {
            Self {
                original_input: input,
                parse_tree,
            }
        })
    }

    /// Attempt to locate this revision in the database
    /// `Ok(None)` is returned if the search fails gracefully
    pub fn try_dereference(&self) -> Result<Option<Hash>, String> {
        match &self.parse_tree {
            RevisionSpecParseTree::HashOrRef(string) => Ok(Some(Hash::Hash(string.clone()))), // TODO don't assume the input is a valid hash
            _ => todo!(),
        }
    }

    /// Attempt to locate this revision in the database;
    /// return the hash or report why it couldn't be found
    pub fn dereference(&self) -> Result<Hash, String> {
        self.try_dereference().and_then(|hash| {
            match hash {
                Some(x) => Ok(x),
                None => Err(format!("Not a valid object name: `{}`", self.original_input)),
            }
        })
    }
}

enum RevisionSpecParseTree {
    // Many strings are ambiguous and could be either hashes or refs;
    // we can't know until checking the database, which happens after
    // this struct is instantiated.
    HashOrRef(String),
    NthParent(Box<RevisionSpecParseTree>, usize), // <rev>^[<n>]
    NthGenerationalParent(Box<RevisionSpecParseTree>, usize), // <rev>~[<n>]
}

impl RevisionSpecParseTree {
    fn parse(input: &str) -> Result<Self, String> {
        Ok(Self::HashOrRef(String::from(input))) // TODO actually parse here
    }
}
