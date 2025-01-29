mod helpers;

use helpers::*;

fn assert_dir_is_proper_clean_repo(repo_dir: &std::path::Path) {

    const EXPECTED_DIRECTORIES: &[&'static str] = &[
        ".mush",
        ".mush/objects",
        ".mush/refs",
    ];

    const EXPECTED_FILES: &[&'static str] = &[
        ".mush/config",
        ".mush/HEAD",
    ];

    EXPECTED_DIRECTORIES.iter().for_each(|dir| {
        assert_directory_exists(
        &repo_dir.join(dir)
        );
    });

    EXPECTED_FILES.iter().for_each(|file| {
        assert_file_exists(
        &repo_dir.join(file)
        );
    });

    // TODO
}

#[test]
fn cwd_init() {
    let dir = tempdir();

    assert!(
        mush!(dir)
            .arg("init")
            .status()
            .unwrap()
            .success()
    );

    assert_dir_is_proper_clean_repo(dir.path());
}
