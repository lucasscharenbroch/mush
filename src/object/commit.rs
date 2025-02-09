use std::str::FromStr;

use chrono::TimeZone;
use itertools::Itertools;

use crate::{cli::CliResult, config::User, hash::Hash};

use super::Object;

const DATE_FORMAT_STRING: &'static str = "%s %:z";

struct PersonTime {
    name: String,
    email: String,
    timestamp: chrono::DateTime<chrono::FixedOffset>,
}

impl PersonTime {
    fn to_string(&self) -> String {
        format!("{} <{}> {}", self.name, self.email, self.timestamp.format(DATE_FORMAT_STRING))
    }
}

pub struct CommitObject {
    tree_hash: crate::hash::Hash,
    parent_hashes: Vec<crate::hash::Hash>,
    author: PersonTime,
    //< git also has committer
    message: String,
}

impl CommitObject {
    pub fn to_string(&self) -> String {
        [
            format!("tree {}\n", self.tree_hash.to_string()),
            self.parent_hashes.iter()
                .map(|hash| format!("parent {}\n", hash.to_string()))
                .collect(),
            format!("author {}\n", self.author.to_string()),
            String::from("\n"),
            self.message.clone(),
        ].join("")
    }

    pub fn from_string(string: &str) -> CliResult<Self> {
        fn from_header_and_message(header: &str, message: &str) -> CliResult<CommitObject>{
            let field_names_to_args_map = header.split("\n")
                .map(|field| {
                    let mut space_separated_strings = field.split(" ");
                    (space_separated_strings.next(), space_separated_strings.collect::<Vec<_>>())
                })
                .into_group_map();

            const FIELD_SPECS: &[(&str, usize, bool, bool)] = &[
                // (field_name, num_args, required, allow_duplicates)
                ("tree", 1, true, false),
                ("parent", 1, false, true),
                ("author", 4, false, true),
            ];

            // verify that all fields match a spec (including num args and duplicates)
            for (key, vals) in field_names_to_args_map.iter() {
                match key {
                    None => return Err(String::from("Malformed commit object: field without a key")),
                    Some(f) => {
                        let spec = FIELD_SPECS.iter()
                            .filter(|spec| spec.0 == *f)
                            .next()
                            .ok_or(format!("Malformed commit object: bad field: {}", f))?;

                        let bad_duplicate_exists = vals.len() > 1 && !spec.3;
                        let bad_arity_exists = vals.iter()
                            .any(|args| args.len() != spec.1);

                        if bad_duplicate_exists {
                            return Err(format!("Malformed commit object: duplicate field: {}", f))
                        }
                        if bad_arity_exists {
                            return Err(format!("Malformed commit object: wrong arity for field: {}", f))
                        }
                    }
                }
            }

            let tree_hash_str = field_names_to_args_map.get(&Some("tree"))
                .unwrap_or(&Vec::new())
                .get(0)
                .ok_or("Malformed commit object: missing tree")?
                [0];

            let tree_hash = Hash::from_bytes(
                tree_hash_str.as_bytes().try_into()
                    .map_err(|_| format!("Malformed commit object: bad hash: {}", tree_hash_str))?
            );

            let parent_hashes = field_names_to_args_map.get(&Some("parent"))
                .unwrap_or(&Vec::new())
                .iter()
                .map(|args| args[0])
                .map(|hash_str| Ok::<_, String>(Hash::from_bytes(
                    hash_str.as_bytes().try_into()
                        .map_err(|_| format!("Malformed commit object: bad hash: {}", tree_hash_str))?
                )))
                .collect::<Result<Vec<_>, _>>()?;

            let [name, email, seconds, offset] = field_names_to_args_map.get(&Some("author"))
                .unwrap_or(&Vec::new())
                .get(0)
                .ok_or("Malformed commit object: missing author")?
                [0..4]
                else {
                    panic!("Violated arity invariant")
                };

            let seconds = seconds.parse::<i64>()
                .map_err(|_| String::from("Malformed commit object: bad timestamp"))?;

            let offset = chrono::FixedOffset::from_str(offset)
                .map_err(|_| String::from("Malformed commit object: bad timestamp"))?;

            let author = PersonTime {
                name: String::from(name),
                email: String::from(email),
                timestamp: offset.timestamp_opt(seconds, 0).single()
                    .ok_or(String::from("Malformed commit object: bad timestamp"))?,
            };

            Ok(CommitObject {
                tree_hash,
                parent_hashes,
                author,
                message: String::from(message),
            })
        }

        if let [header, message] = string.splitn(2, "\n\n").collect::<Vec<_>>()[..] {
            from_header_and_message(header, message)
        } else {
            Err(String::from("Malformed commit object: no double newline found"))
        }
    }

    /// Assumes that all supplied hashes are valid.
    /// (they will be place into the database without being checked)
    pub fn new(tree_hash: Hash, parent_hashes: Vec<Hash>, user: User, message: String) -> Self {
        CommitObject {
            tree_hash,
            parent_hashes,
            author: PersonTime {
                name: user.name,
                email: user.email,
                timestamp: chrono::Local::now().into(),
            },
            message
        }
    }
}

impl Into<Object<'static>> for CommitObject {
    fn into(self) -> Object<'static> {
        Object::Commit(self)
    }
}
