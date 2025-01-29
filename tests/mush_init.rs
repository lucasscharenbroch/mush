mod helpers;

use mush as src;

use helpers::*;

fn assert_dir_is_proper_clean_repo() {
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
    )
}
