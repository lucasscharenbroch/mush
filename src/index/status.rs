use std::fs::ReadDir;

use crate::{cli::CliResult, object::tree::{FilenameTree, ObjectTree}};

use super::{Index, RepoRelativeFilename};

pub enum FileOrDir {
	File(RepoRelativeFilename),
	Dir(RepoRelativeFilename),
}

impl std::ops::Deref for FileOrDir {
	type Target = RepoRelativeFilename;
	fn deref(&self) -> &Self::Target {
		match self {
			Self::File(name) => name,
			Self::Dir(name) => name,
		}
	}
}

pub enum StagedChangeType {
	Add, // "new file"
	Modify, // "modified"
	Delete, // "deleted"
}

impl std::fmt::Display for StagedChangeType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Add => f.write_str("new file"),
			Self::Modify => f.write_str("modified"),
			Self::Delete => f.write_str("deleted"),
		}
	}
}

#[derive(Clone)]
pub enum UnstagedChangeType {
	Modify, // "modified"
	Delete, // "deleted"
}

impl Into<StagedChangeType> for UnstagedChangeType {
	fn into(self) -> StagedChangeType {
		match self {
			Self::Modify => StagedChangeType::Modify,
			Self::Delete => StagedChangeType::Delete,
		}
	}
}

impl std::fmt::Display for UnstagedChangeType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<UnstagedChangeType as Into<StagedChangeType>>::into(self.clone()).fmt(f)
	}
}

/// Porcelain helper. Stores a conceptual diff between the
/// index and the working tree. Internal representation of
/// the output of `mush status`.
pub struct IndexStatus {
	pub staged_changes: Vec<(StagedChangeType, RepoRelativeFilename)>,
	pub unstaged_changes: Vec<(UnstagedChangeType, RepoRelativeFilename)>,
	pub untracked_files: Vec<FileOrDir>,
}

impl IndexStatus {
	pub fn create_from_index_and_working_tree(index: Index, working_tree: ReadDir) -> CliResult<Self> {
        let index_object_tree = FilenameTree::from_index(index).into_object_tree()?;


		// todo!()
		Ok(IndexStatus {
			staged_changes: vec![(StagedChangeType::Add, RepoRelativeFilename("abc".to_string())), (StagedChangeType::Modify, RepoRelativeFilename("xyz".to_string()))],
			unstaged_changes: vec![(UnstagedChangeType::Delete, RepoRelativeFilename("abc".to_string())), (UnstagedChangeType::Modify, RepoRelativeFilename("xyz".to_string()))],
			untracked_files: vec![FileOrDir::File(RepoRelativeFilename("efg".to_string())), FileOrDir::Dir(RepoRelativeFilename("hij".to_string()))],
		})
	}
}
