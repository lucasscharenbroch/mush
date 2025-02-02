mod helpers;

use helpers::*;

#[test]
fn hash_file_write_and_cat_file_in_repository_subdirectory() {
    let dir = tempdir();
    let subdir = dir.path().join("subdirectory-name");
    create_dir(subdir.as_path());
    mush_init_clean_repo(&dir);

    const FILES: &[(&str, &str, &str, &[u8])] = &[
        ("246.txt", "pedal\nstroke\nmush\n246\n", "6b71b21eca93ae1244825d70173ce4185efabe27",
            &[0x78, 0x01, 0x4b, 0xca, 0xc9, 0x4f, 0x52, 0x30, 0x32, 0x62, 0x28, 0x48,
                0x4d, 0x49, 0xcc, 0xe1, 0x2a, 0x2e, 0x29, 0xca, 0xcf, 0x4e, 0xe5, 0xca,
                0x2d, 0x2d, 0xce, 0xe0, 0x32, 0x32, 0x31, 0xe3, 0x02, 0x00, 0x97, 0x08,
                0x09, 0x43]),
    ];

    FILES.iter().for_each(|(filename, contents, hash, _object)| {
        create_file_with_contents(dir.path(), filename, contents);

        let output = mush!(subdir)
                .arg("hash-object")
                .arg("-w")
                .arg(format!("../{filename}"))
                .output()
                .unwrap();

        assert_output_success(&output);
        assert_eq!(
            format!("{hash}\n"),
            String::from_utf8(output.stdout).unwrap()
        );

        let output = mush!(subdir)
                .arg("cat-file")
                .arg("-p")
                .arg(hash)
                .output()
                .unwrap();

        assert_output_success(&output);
        assert_eq!(
            format!("{contents}"),
            String::from_utf8(output.stdout).unwrap()
        );
    });

    FILES.iter().for_each(|(_filename, _contents, hash, object)| {
        let (prefix, suffix) = hash.split_at(2);

        assert_file_contents(
            &dir.path().join( format!(".mush/objects/{}/{}", prefix, suffix)),
            object.to_vec()
        )
    });
}
