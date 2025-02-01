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

#[test]
fn various_dirs_init() {
    const INIT_DIRS: &[&'static str] = &[
        "./",
        "./a",
        "./abc",
        "./abc-def.g",
        "a",
        "abc",
        "abc-def.g",
    ];

    INIT_DIRS.iter().for_each(|init_dir| {
        let dir = tempdir();

        assert!(
            mush!(dir)
                .arg("init")
                .arg(init_dir)
                .status()
                .unwrap()
                .success()
        );

        assert_dir_is_proper_clean_repo(&dir.path().join(init_dir));
    });
}

#[test]
fn various_nested_dirs_init() {
    const INIT_DIRS: &[&'static str] = &[
        "./a/b/c",
        "./a/b/c/d/e/f",
        "./ab/cd/e/f",
    ];

    INIT_DIRS.iter().for_each(|init_dir| {
        let dir = tempdir();

        assert!(
            mush!(dir)
                .arg("init")
                .arg(init_dir)
                .status()
                .unwrap()
                .success()
        );

        assert_dir_is_proper_clean_repo(&dir.path().join(init_dir));
    });
}
