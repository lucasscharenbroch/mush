extern crate tempdir;

use std::io::Write;

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

// macro to allow multiple types for $directory (path, tempdir, string)
pub fn mush_init_clean_repo(directory: &tempdir::TempDir) {
    assert!(
        mush!(directory)
            .arg("init")
            .status()
            .unwrap()
            .success()
    );
}

pub fn tempdir() -> tempdir::TempDir {
    tempdir::TempDir::new("mush-test")
        .expect("failed to create temp directory for test")
}

pub fn assert_directory_exists(path: &std::path::Path) {
    assert_eq!(
        true,
        std::fs::metadata(path)
            .unwrap()
            .is_dir()
    )
}

pub fn assert_file_exists(path: &std::path::Path) {
    assert_eq!(
        true,
        std::fs::metadata(path)
            .unwrap()
            .is_file()
    )
}

pub fn assert_file_contents(path: &std::path::Path, contents: Vec<u8>) {
    assert_eq!(
        std::fs::read(path)
            .unwrap(),
        contents
    )
}

pub fn create_file_with_contents(directory: &std::path::Path, filename: &str, contents: &str) {
    let mut file = std::fs::File::create(directory.join(filename)).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn create_dir(directory: &std::path::Path) {
    std::fs::create_dir(directory).unwrap();
}

pub fn assert_output_success(output: &std::process::Output) {
    assert!(output.status.success(), "stderr = ```{}```", String::from_utf8(output.stderr.clone()).unwrap());
}
