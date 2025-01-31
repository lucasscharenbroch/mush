mod helpers;

use std::{io::Write, process::Stdio};

use helpers::*;

#[test]
fn hash_files() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    [
        ("246.txt", "pedal\nstroke\nmush\n246\n", "6b71b21eca93ae1244825d70173ce4185efabe27"),
        ("375.txt", "pedal\nstroke\nmush\n375\n", "0aee910e6bc8fc564b3d7e8004bdefb76296d512"),
        ("403.txt", "pedal\nstroke\nmush\n403\n", "0ab24bc4ce4fa08fa0f8c12e8001db7e9b936ccd"),
        ("999.txt", "pedal\nstroke\nmush\n999\n", "99622a960c9f3d0232df4d90149b666c11159b9e"),
    ].iter().for_each(|(filename, contents, hash)| {
        create_file_with_contents(dir.path(), filename, contents);

        let output = mush!(dir)
                .arg("hash-object")
                .arg(filename)
                .output()
                .unwrap();

        assert!(output.status.success());
        assert_eq!(
            format!("{hash}\n"),
            String::from_utf8(output.stdout).unwrap()
        );
    });
}

#[test]
fn write_files() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    const FILES: &[(&str, &str, &str, &[u8])] = &[
        ("246.txt", "pedal\nstroke\nmush\n246\n", "6b71b21eca93ae1244825d70173ce4185efabe27",
            &[0x78, 0x01, 0x4b, 0xca, 0xc9, 0x4f, 0x52, 0x30, 0x32, 0x62, 0x28, 0x48,
                0x4d, 0x49, 0xcc, 0xe1, 0x2a, 0x2e, 0x29, 0xca, 0xcf, 0x4e, 0xe5, 0xca,
                0x2d, 0x2d, 0xce, 0xe0, 0x32, 0x32, 0x31, 0xe3, 0x02, 0x00, 0x97, 0x08,
                0x09, 0x43]),
        ("375.txt", "pedal\nstroke\nmush\n375\n", "0aee910e6bc8fc564b3d7e8004bdefb76296d512",
            &[0x78, 0x01, 0x4b, 0xca, 0xc9, 0x4f, 0x52, 0x30, 0x32, 0x62, 0x28, 0x48,
                0x4d, 0x49, 0xcc, 0xe1, 0x2a, 0x2e, 0x29, 0xca, 0xcf, 0x4e, 0xe5, 0xca,
                0x2d, 0x2d, 0xce, 0xe0, 0x32, 0x36, 0x37, 0xe5, 0x02, 0x00, 0x97, 0x13,
                0x09, 0x46]),
        ("403.txt", "pedal\nstroke\nmush\n403\n", "0ab24bc4ce4fa08fa0f8c12e8001db7e9b936ccd",
            &[0x78, 0x01, 0x4b, 0xca, 0xc9, 0x4f, 0x52, 0x30, 0x32, 0x62, 0x28, 0x48,
                0x4d, 0x49, 0xcc, 0xe1, 0x2a, 0x2e, 0x29, 0xca, 0xcf, 0x4e, 0xe5, 0xca,
                0x2d, 0x2d, 0xce, 0xe0, 0x32, 0x31, 0x30, 0xe6, 0x02, 0x00, 0x96, 0xfe,
                0x09, 0x3e]),
        ("999.txt", "pedal\nstroke\nmush\n999\n", "99622a960c9f3d0232df4d90149b666c11159b9e",
            &[0x78, 0x01, 0x4b, 0xca, 0xc9, 0x4f, 0x52, 0x30, 0x32, 0x62, 0x28, 0x48,
                0x4d, 0x49, 0xcc, 0xe1, 0x2a, 0x2e, 0x29, 0xca, 0xcf, 0x4e, 0xe5, 0xca,
                0x2d, 0x2d, 0xce, 0xe0, 0xb2, 0xb4, 0xb4, 0xe4, 0x02, 0x00, 0x97, 0x39,
                0x09, 0x52]),
    ];

    FILES.iter().for_each(|(filename, contents, hash, _object)| {
        create_file_with_contents(dir.path(), filename, contents);

        let output = mush!(dir)
                .arg("hash-object")
                .arg("-w")
                .arg(filename)
                .output()
                .unwrap();

        assert!(output.status.success());
        assert_eq!(
            format!("{hash}\n"),
            String::from_utf8(output.stdout).unwrap()
        );
    });

    FILES.iter().for_each(|(_filename, contents, hash, object)| {
        let (prefix, suffix) = hash.split_at(2);

        assert_file_contents(
            &dir.path().join( format!(".mush/objects/{}/{}", prefix, suffix)),
            object.to_vec()
        )
    });
}

#[test]
fn hash_stdin() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    // example from git book https://git-scm.com/book/en/v2/Git-Internals-Git-Objects

    let mut hash_object_process = mush!(dir)
            .arg("hash-object")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

    let mut stdin = hash_object_process.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all("test content\n".as_bytes()).unwrap();
    });

    let output = hash_object_process.wait_with_output().unwrap();

    assert!(output.status.success());
    assert_eq!(
        "d670460b4b4aece5915caf5c68d12f560a9fe3e4\n",
        String::from_utf8(output.stdout).unwrap()
    );
}

#[test]
fn hash_stdin_with_pipe() {
    // try the explicit pipe from echo, just for fun

    let dir = tempdir();
    mush_init_clean_repo(&dir);

    // example from git book https://git-scm.com/book/en/v2/Git-Internals-Git-Objects

    let echo = std::process::Command::new("echo")
            .arg("test content")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

    let output = mush!(dir)
            .arg("hash-object")
            .arg("-")
            .stdin(echo.stdout.unwrap())
            .output()
            .unwrap();

    assert!(output.status.success());
    assert_eq!(
        "d670460b4b4aece5915caf5c68d12f560a9fe3e4\n",
        String::from_utf8(output.stdout).unwrap()
    );
}

#[test]
fn write_stdin() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    // example from git book https://git-scm.com/book/en/v2/Git-Internals-Git-Objects

    let mut hash_object_process = mush!(dir)
            .arg("hash-object")
            .arg("-")
            .arg("-w")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

    let mut stdin = hash_object_process.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all("test content\n".as_bytes()).unwrap();
    });

    let output = hash_object_process.wait_with_output().unwrap();

    assert!(output.status.success());
    assert_eq!(
        "d670460b4b4aece5915caf5c68d12f560a9fe3e4\n",
        String::from_utf8(output.stdout).unwrap()
    );

    assert_file_contents(
        &dir.path().join(".mush/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4"),
        vec![ // "blob 13\0test content\n" compressed with zlib
        //  zlib
        //  vvvv
        //        vvvv no compression/low
            0x78, 0x01, 0x4b, 0xca, 0xc9, 0x4f, 0x52, 0x30, 0x34, 0x66, 0x28, 0x49,
            0x2d, 0x2e, 0x51, 0x48, 0xce, 0xcf, 0x2b, 0x49, 0xcd, 0x2b, 0xe1, 0x02,
            0x00, 0x4b, 0xdf, 0x07, 0x09
        ]
    );
}
