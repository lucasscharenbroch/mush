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

#[test]
fn lots_of_nesting() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    /*
        .
        └── src
            ├── a "1"
            ├── b
            │   ├── c "2"
            │   ├── d
            │   │   ├── g "3"
            │   │   └── h "4"
            │   ├── e "5"
            │   ├── f
            │   │   ├── i "6"
            │   │   ├── j "7"
            │   │   └── m
            │   │       └── n "8"
            │   └── k "9"
            └── l "10"
    */
    create_dir(dir.path(), "src");
    create_dir(dir.path(), "src/b");
    create_dir(dir.path(), "src/b/d");
    create_dir(dir.path(), "src/b/f");
    create_dir(dir.path(), "src/b/f/m");
    create_file_with_contents(dir.path(), "src/a", "1\n");
    create_file_with_contents(dir.path(), "src/b/c", "2\n");
    create_file_with_contents(dir.path(), "src/b/e", "5\n");
    create_file_with_contents(dir.path(), "src/b/d/g", "3\n");
    create_file_with_contents(dir.path(), "src/b/d/h", "4\n");
    create_file_with_contents(dir.path(), "src/b/f/i", "6\n");
    create_file_with_contents(dir.path(), "src/b/f/j", "7\n");
    create_file_with_contents(dir.path(), "src/b/k", "9\n");
    create_file_with_contents(dir.path(), "src/l", "10\n");
    create_file_with_contents(dir.path(), "src/b/f/m/n", "8\n");

    [
        ("src/l", "f599e28b8ab0d8c9c57a486c89c4a5132dcbd3b2"),
        ("src/b/d/g","00750edc07d6415dcc07ae0351e9397b0222b7ba"),
        ("src/b/d/h", "b8626c4cff2849624fb67f87cd0ad72b163671ad"),
        ("src/b/e", "7ed6ff82de6bcc2a78243fc9c54d3ef5ac14da69"),
        ("src/b/c", "0cfbf08886fca9a91cb753ec8734c84fcbe52c9f"),
        ("src/b/f/m/n", "45a4fb75db864000d01701c0f7a51864bd4daabf"),
        ("src/b/f/j", "7f8f011eb73d6043d2e6db9d2c101195ae2801f2"),
        ("src/b/f/i", "1e8b314962144c26d5e0e50fd29d2ca327864913"),
        ("src/b/k", "ec635144f60048986bc560c5576355344005e6e7"),
        ("src/a", "d00491fd7e5bb6fa28c517a0bb32b8b506539d4d")
    ].iter()
        .for_each(|(file, hash)|
            assert!(mush!(dir)
                .arg("update-index")
                .arg("--add")
                .arg(hash)
                .arg(file)
                .output()
                .unwrap()
                .status
                .success()
            ));

    let output = mush!(dir)
        .arg("write-tree")
        .output()
        .unwrap();

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "a1eae5b44e6ebf4cef4a3a45bc7d9b70c1a766a1\n");
    assert!(output.status.success());

    // we know the hashes line up, other tests cover that objects are actually written,
    // so we can be pretty confident that the files line up too. Just check that they exist.

    [
        ".mush/objects/a1/eae5b44e6ebf4cef4a3a45bc7d9b70c1a766a1", // .
        ".mush/objects/22/b4a6817b0752f485ade63402ff63969b506a85", // src
        ".mush/objects/d0/0491fd7e5bb6fa28c517a0bb32b8b506539d4d", // a
        ".mush/objects/f6/55e01806bdbbd07fecc96e92264dc36fa1f87f", // b
        ".mush/objects/0c/fbf08886fca9a91cb753ec8734c84fcbe52c9f", // c
        ".mush/objects/d6/916ac80889bef0a904686b948974f3c3684c99", // d
        ".mush/objects/7e/d6ff82de6bcc2a78243fc9c54d3ef5ac14da69", // e
        ".mush/objects/d2/d468c6b2356b420d9c1f0a55defcf7f5eac9a9", // f
        ".mush/objects/00/750edc07d6415dcc07ae0351e9397b0222b7ba", // g
        ".mush/objects/b8/626c4cff2849624fb67f87cd0ad72b163671ad", // h
        ".mush/objects/1e/8b314962144c26d5e0e50fd29d2ca327864913", // i
        ".mush/objects/7f/8f011eb73d6043d2e6db9d2c101195ae2801f2", // j
        ".mush/objects/ec/635144f60048986bc560c5576355344005e6e7", // k
        ".mush/objects/f5/99e28b8ab0d8c9c57a486c89c4a5132dcbd3b2", // l
        ".mush/objects/f3/98d19e05257caa1473957cd7b6b8058771cfe4", // m
        ".mush/objects/45/a4fb75db864000d01701c0f7a51864bd4daabf", // n
    ].iter()
            .for_each(|file| assert_file_exists(&dir.path().join(file)));
}
