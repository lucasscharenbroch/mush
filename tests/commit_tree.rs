mod helpers;

use std::{fs::File, process::Stdio};

use helpers::*;
use mush::hash::Hash;

#[test]
fn one_commit_full_pipeline() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

	assert!(
		mush!(dir)
			.arg("config")
			.arg("user.name")
			.arg("oops")
			.output()
			.unwrap()
			.status
			.success()
	);

	assert!(
		mush!(dir)
			.arg("config")
			.arg("user.name")
			.arg("Bud Weiser")
			.output()
			.unwrap()
			.status
			.success()
	);

	assert!(
		mush!(dir)
			.arg("config")
			.arg("user.email")
			.arg("bud@wiser.org")
			.output()
			.unwrap()
			.status
			.success()
	);

	create_dir(dir.path(), "x");
	create_file_with_contents(dir.path(), "x/a", "a\n");
	create_file_with_contents(dir.path(), "b", "b\n");

	let a_hash = "78981922613b2afb6025042ff6bd878ac1994e85";
	let b_hash = "63d8dbd40c23542e740659a7168a0ce3138ea748";

	assert!(
		mush!(dir)
			.arg("update-index")
			.arg("--add")
			.arg(a_hash)
			.arg("x/a")
			.output()
			.unwrap()
			.status
			.success()
	);

	assert!(
		mush!(dir)
			.arg("update-index")
			.arg("--add")
			.arg(b_hash)
			.arg("b")
			.output()
			.unwrap()
			.status
			.success()
	);

	let output = mush!(dir)
		.arg("write-tree")
		.output()
		.unwrap();

	assert_output_success(&output);
	assert_eq!("b923cc8e0ff559cf73c8302c7516a1364333bdfa\n".as_bytes(), output.stdout);

    let echo = std::process::Command::new("echo")
            .arg("messsage")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

	let output = mush!(dir)
		.arg("commit-tree")
		.arg("b923cc8e0ff559cf73c8302c7516a1364333bdfa")
		.stdin(echo.stdout.unwrap())
		.output()
		.unwrap();

	assert_output_success(&output);

	// it probably would be safer to manually assert the contents
	// of the commit object file, but the timestamp changes, and might vary
	// in character length (I don't know if the timezone is always a fixed
	// char size). So I don't want to try to write matchers for that, so let's
	// just read the object using the library code, and assert some things about
	// that. This way, the test can stay the same if the reading/writing
	// code changes (which is possible but unlikely).

	let commit_hash = Hash::try_from_str(String::from_utf8_lossy(output.stdout.as_slice()).trim())
		.unwrap();

	std::env::set_current_dir(dir.path()).unwrap();
	let commit_object = mush::io::read_object(&commit_hash).unwrap();

	if let mush::object::Object::Commit(commit) = commit_object {
		assert_eq!("messsage\n", commit.message);
		assert_eq!("b923cc8e0ff559cf73c8302c7516a1364333bdfa", commit.tree_hash.as_str());
		assert_eq!(0, commit.parent_hashes.len());
		assert_eq!("Bud Weiser", commit.author.name);
		assert_eq!("bud@wiser.org", commit.author.email);
		// assume the date/time is right; it's kind of hard to construct a datetime that isn't the current time
	} else {
		assert!(false)
	}
}