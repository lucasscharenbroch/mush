mod helpers;

use helpers::*;
use mush::hash::Hash;

#[test]
fn once_nested() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    create_dir(dir.path(), "y");
    create_file_with_contents(dir.path(), "y/xyz.txt", "abc\n");

    let hash = Hash::from_bytes(
        mush!(dir)
            .arg("hash-object")
            .arg("y/xyz.txt")
            .output()
            .unwrap()
            .stdout[..20]
            .try_into()
            .unwrap()
    );

    assert!(
        mush!(dir)
            .arg("update-index")
            .arg("--add")
            .arg(hash.as_str())
            .arg("y/xyz.txt")
            .output()
            .unwrap()
            .status.success()
    );


    let output = mush!(dir)
        .arg("write-tree")
        .output()
        .unwrap();

    let expected_contents = &[
        0x78, 0x01, 0x01, 0x24, 0x00, 0xdb, 0xff, 0x74, 0x72, 0x65, 0x65, 0x20,
        0x32, 0x38, 0x00, 0x34, 0x30, 0x30, 0x30, 0x30, 0x20, 0x79, 0x00, 0x89,
        0x2b, 0x8c, 0x36, 0xb1, 0x57, 0x9b, 0x89, 0x3c, 0x2e, 0xb0, 0x56, 0x41,
        0xd4, 0x36, 0x1b, 0xd2, 0x5f, 0xfd, 0xe9, 0xcb, 0xaf, 0x0d, 0x57
    ];

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "f48f697b8d1ff3a07132fb982912499cf26d0f68\n");

    assert!(output.status.success());
    assert_file_contents(
        &dir.path().join(".mush/objects/f4/8f697b8d1ff3a07132fb982912499cf26d0f68"),
        expected_contents,
    );
}
