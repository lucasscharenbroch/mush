extern crate tempdir;

use mush as src;

// abbreviates the finding of the executable and attaching the tempdir
// also makes it less likely to accidentally use the real cwd
// this is a macro because it returns (&mut Command), which causes
// ownership issues
// mush(directory: tempdir::TempDir) -> &mut std::process::Command

#[macro_export]
macro_rules! mush {
    ($directory:expr) => {
        std::process::Command::new(
            std::path::Path::new("./target/debug/mush").canonicalize()
                .expect("failed to canonicalize mush executable path")
        ).current_dir(&$directory)
    };
}

pub fn tempdir() -> tempdir::TempDir {
    tempdir::TempDir::new("mush-test")
        .expect("failed to create temp directory for test")
}

pub fn assert_directory_exists(path: &std::path::Path) {
    assert_eq!(
        true,
        std::fs::metadata(path)
                .expect("expected path to be directory")
                .is_dir()
    )
}

pub fn assert_file_exists(path: &std::path::Path) {
    assert_eq!(
        true,
        std::fs::metadata(path)
                .expect("expected path to be directory")
                .is_file()
    )
}
