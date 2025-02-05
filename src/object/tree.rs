use std::ffi::OsStr;

use crate::{cli::CliResult, index::{Index, IndexEntry, RepoRelativeFilename}, io::write_object};

use itertools::Itertools;

use super::Object;

/// A n-ary tree whose nodes are objects (leaves are blobs, non-leaves are "trees"
/// (a better name for "tree objects" would be "tree node objects")).
/// Technically, in the case of the empty tree-object, a tree can also be leaf
/// (a node with an empty vector of children).
pub enum ObjectTree {
    Leaf(Object<'static>),
    Node(Object<'static>, Vec<ObjectTree>),
}

impl ObjectTree {
    pub fn root(&self) -> &Object<'static> {
        match self {
            Self::Leaf(object) => object,
            Self::Node(object, _children) => object,
        }
    }

    pub fn write(&self) -> CliResult<()> {
        match &self {
            Self::Leaf(object) => {
                write_object(object)
            },
            Self::Node(object, children) => {
                children.iter()
                    .map(|child| child.write())
                    .collect::<CliResult<()>>()
                    .and_then(|_| write_object(object))
            },
        }
    }
}

/// A `ObjectTree`, modulo the objects (purely computed from an index)
pub struct FilenameTree {
    // This is a little asymetrical with `ObjectTree`, which is just an enum.
    // While the root of the `FilenameTree` is conceptually a `Node`, it doesn't
    // have a filename.
    // Every node in an object tree has an associated object.
    // Only non-root nodes of filename trees have associated file (or directory) names.
    nodes: Vec<FilenameTreeNode>
}

pub enum FilenameTreeNode {
    Leaf(RepoRelativeFilename),
    Node(String, Vec<FilenameTreeNode>),
}

impl FilenameTreeNode {
    fn file_str(&self) -> &str {
        match self {
            FilenameTreeNode::Leaf(filename) => filename,
            FilenameTreeNode::Node(dir_name, _nodes) => dir_name,
        }
    }
}

impl FilenameTree {
    /// Produces a tree of tree objects ("tree objects" would be more accurately named "tree-node objects").
    pub fn from_index(index: Index) -> Self {
        fn entries_to_tree_nodes(entries: impl Iterator<Item = IndexEntry>) -> Vec<FilenameTreeNode> {
            let mut directory_to_entries_map = entries
                .map(|mut entry| {
                    let file_name_path = std::path::Path::new(OsStr::new(entry.file_name.as_str()));
                    let directory = file_name_path.components()
                        .dropping_back(1) // ignore filename component
                        .next()
                        .map(|component| <std::path::Component<'_> as AsRef<std::path::Path>>::as_ref(&component).to_owned());

                    if let Some(ref dir_path) = directory {
                        entry.file_name = RepoRelativeFilename(String::from(
                            file_name_path.strip_prefix(dir_path).unwrap().to_str().unwrap().clone()
                        ));
                    }
                    (directory, entry)
                })
                .into_group_map();

            // create blobs from entries with no parent directory.
            let blobs = directory_to_entries_map.remove(&None).unwrap_or(vec![])
                .into_iter()
                .map(|entry| FilenameTreeNode::Leaf(entry.file_name))
                .collect::<Vec<_>>();

            // create trees (recursively) with the rest.
            let trees = directory_to_entries_map.into_iter()
                .map(|(dir_name, dir_entries)|
                    FilenameTreeNode::Node(
                        String::from(dir_name.unwrap().to_str().unwrap()),
                        entries_to_tree_nodes(dir_entries.into_iter()),
                    )
                );

            blobs.into_iter().chain(trees.into_iter())
                .sorted_by_key(|node| String::from(node.file_str()))
                .collect()
        }

        FilenameTree {
            nodes: entries_to_tree_nodes(index.into_entries().into_values())
        }
    }

    // This is a bad function name for readability, and it doesn't really add any meaning but I like it.
    // And, let's face it, nobody will be reading this code except me.
    /// Convert to an `ObjectTree`
    pub fn objectify(self) -> ObjectTree {
        todo!()
    }
}
