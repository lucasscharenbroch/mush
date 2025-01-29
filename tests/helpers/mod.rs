extern crate tempdir;

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

