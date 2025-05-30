use crate::{cli::CliResult, hash::Hash};

/// A hypothetical pointer to an object (yet to be `dereference`d)
/// (could be a hash, ref, or expression involving the two)
pub struct RevisionSpec<'s> {
    original_input: &'s str,
    parse_tree: RevisionSpecParseTree,
}

impl<'s> RevisionSpec<'s> {
    pub fn parse(input: &'s str) -> CliResult<Self> {
        RevisionSpecParseTree::parse(input).map(|parse_tree| Self {
            original_input: input,
            parse_tree,
        })
    }

    /// Attempt to locate this revision in the database
    /// `Ok(None)` is returned if the search fails gracefully
    pub fn try_dereference(&self) -> CliResult<Option<Hash>> {
        match &self.parse_tree {
            RevisionSpecParseTree::HashOrRef(string) => Ok(Some(Hash::try_from_str(string).unwrap())), // TODO don't assume the input is a valid hash
            _ => todo!(),
        }
    }

    /// Attempt to locate this revision in the database;
    /// return the hash or report why it couldn't be found
    pub fn dereference(&self) -> CliResult<Hash> {
        self.try_dereference().and_then(|hash| match hash {
            Some(x) => Ok(x),
            None => Err(format!(
                "Not a valid object name: `{}`",
                self.original_input
            )),
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
    fn parse(input: &str) -> CliResult<Self> {
        let main_re = regex::Regex::new(r"^(?<base>[^^~]+)(?<modifiers>[\^~].*)?").unwrap();

        // e.g.
        // origin/master~3^2~
        // ^^^^^^^^^^^^^
        // "base"
        //              ^^^^
        //              "modifiers"

        main_re
            .captures(input)
            .and_then(|captures| {
                let base_str = &captures["base"];
                let modifiers_str = captures.name("modifiers")
                    .map(|matcch| matcch.as_str())
                    .unwrap_or("");

                let mod_re = regex::Regex::new(r"([\^~])([0-9]*)").unwrap();

                let modifiers_opt = mod_re
                    .captures_iter(modifiers_str)
                    .map(|captures| captures.extract())
                    .map(|(_, [operator, arg])| {
                        let arg = if arg == "" { "1" } else { arg };
                        arg.parse::<usize>()
                            // Result to Option
                            .map(|n| Some(n))
                            .unwrap_or(None)
                            .map(|n| (operator, n))
                    })
                    .collect::<Option<Vec<_>>>(); // sequence (fail on first None)

                modifiers_opt.map(|modifiers| {
                    modifiers.iter().fold(
                        Self::HashOrRef(String::from(base_str)),
                        |acc, (operator, n)| match *operator {
                            "^" => RevisionSpecParseTree::NthParent(Box::new(acc), *n),
                            "~" => RevisionSpecParseTree::NthGenerationalParent(Box::new(acc), *n),
                            _ => panic!("Invariant error (regex with invalid modifier modifier)"),
                        },
                    )
                })
            })
            .ok_or(format!("Bad revision spec: `{input}`"))
    }
}
