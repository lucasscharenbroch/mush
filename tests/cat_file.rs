mod helpers;

use helpers::*;
use mush::hash::Hash;

#[test]
fn exists() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    [
        ("grocery-list.txt", "- eggs\n- pork roll\n- cheese\n- kaiser bun\n", "64dee893c45dee4f826bd62978abebf31c3cdcba", true),
        ("999.txt", "pedal\nstroke\nmush\n999\n", "99622a960c9f3d0232df4d90149b666c11159b9e", false),
        ("cliche.breakdown", "They will not win. They were not there for hills at greenbush", "638d015b06cf20efa84b7f47b91f73acb19a7c05", false),
        ("goggins", concat!("As I lay there, curled up in the tub, shivering in the fetal position, ",
            "relishing the pain, I thought of something else too. If I could run 101 miles ",
            "with zero training, imagine what I could do with a little preparation."),
            "da9efed46f063d0328de76ca16070378b9bb8eb3", true),
    ].iter().for_each(|(filename, contents, hash, should_write)| {
        create_file_with_contents(dir.path(), filename, contents);

        let output = mush!(dir)
                .arg("hash-object")
                .arg(filename)
                .args(["-w"].iter().filter(|_| *should_write))
                .output()
                .unwrap();

        assert_output_success(&output);
        assert_eq!(
            format!("{hash}\n"),
            String::from_utf8(output.stdout).unwrap()
        );

        let output = mush!(dir)
                .arg("cat-file")
                .arg("-e")
                .arg(hash)
                .output()
                .unwrap();

        assert_eq!(*should_write, output.status.success());
    });
}

#[test]
fn tipe() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    [
        ("grocery-list.txt", "- eggs\n- pork roll\n- cheese\n- kaiser bun\n", "64dee893c45dee4f826bd62978abebf31c3cdcba", "blob"),
    ].iter().for_each(|(filename, contents, hash, typename)| {
        create_file_with_contents(dir.path(), filename, contents);

        let output = mush!(dir)
                .arg("hash-object")
                .arg(filename)
                .arg("-w")
                .output()
                .unwrap();

        assert_output_success(&output);
        assert_eq!(
            format!("{hash}\n"),
            String::from_utf8(output.stdout).unwrap()
        );

        let output = mush!(dir)
                .arg("cat-file")
                .arg("-t")
                .arg(hash)
                .output()
                .unwrap();

        assert_output_success(&output);
        assert_eq!(
            format!("{typename}\n"),
            String::from_utf8(output.stdout).unwrap()
        );


    });
}

#[test]
fn size() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    [
        ("grocery-list.txt", "- eggs\n- pork roll\n- cheese\n- kaiser bun\n", "64dee893c45dee4f826bd62978abebf31c3cdcba", 41),
        ("999.txt", "pedal\nstroke\nmush\n999\n", "99622a960c9f3d0232df4d90149b666c11159b9e", 22),
        ("cliche.breakdown", "They will not win. They were not there for hills at greenbush", "638d015b06cf20efa84b7f47b91f73acb19a7c05", 61),
        ("goggins", concat!("As I lay there, curled up in the tub, shivering in the fetal position, ",
            "relishing the pain, I thought of something else too. If I could run 101 miles ",
            "with zero training, imagine what I could do with a little preparation."),
            "da9efed46f063d0328de76ca16070378b9bb8eb3", 219),
    ].iter().for_each(|(filename, contents, hash, size_in_bytes)| {
        create_file_with_contents(dir.path(), filename, contents);

        let output = mush!(dir)
                .arg("hash-object")
                .arg(filename)
                .arg("-w")
                .output()
                .unwrap();

        assert_output_success(&output);
        assert_eq!(
            format!("{hash}\n"),
            String::from_utf8(output.stdout).unwrap()
        );

        let output = mush!(dir)
                .arg("cat-file")
                .arg("-s")
                .arg(hash)
                .output()
                .unwrap();

        assert_output_success(&output);
        assert_eq!(
            format!("{size_in_bytes}\n"),
            String::from_utf8(output.stdout).unwrap()
        );
    });
}

#[test]
fn pretty_blob() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    [
        ("grocery-list.txt", "- eggs\n- pork roll\n- cheese\n- kaiser bun\n", "64dee893c45dee4f826bd62978abebf31c3cdcba"),
        ("999.txt", "pedal\nstroke\nmush\n999\n", "99622a960c9f3d0232df4d90149b666c11159b9e"),
        ("cliche.breakdown", "They will not win. They were not there for hills at greenbush", "638d015b06cf20efa84b7f47b91f73acb19a7c05"),
        ("goggins", concat!("As I lay there, curled up in the tub, shivering in the fetal position, ",
            "relishing the pain, I thought of something else too. If I could run 101 miles ",
            "with zero training, imagine what I could do with a little preparation."),
            "da9efed46f063d0328de76ca16070378b9bb8eb3"),
    ].iter().for_each(|(filename, contents, hash)| {
        create_file_with_contents(dir.path(), filename, contents);

        let output = mush!(dir)
                .arg("hash-object")
                .arg(filename)
                .arg("-w")
                .output()
                .unwrap();

        assert_output_success(&output);
        assert_eq!(
            format!("{hash}\n"),
            String::from_utf8(output.stdout).unwrap()
        );

        let output = mush!(dir)
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
}

#[test]
fn pretty_tree() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    create_dir(dir.path(), "y");
    create_file_with_contents(dir.path(), "y/xyz.txt", "abc\n");
    create_file_with_contents(dir.path(), "x", "abcd\n");

    assert!(
        mush!(dir)
            .arg("update-index")
            .arg("--add")
            .arg("acbe86c7c89586e0912a0a851bacf309c595c308")
            .arg("x")
            .output()
            .unwrap()
            .status.success()
    );

    assert!(
        mush!(dir)
            .arg("update-index")
            .arg("--add")
            .arg("8baef1b4abc478178b004d62031cf7fe6db6f903")
            .arg("y/xyz.txt")
            .output()
            .unwrap()
            .status.success()
    );

    let output = mush!(dir)
        .arg("write-tree")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "0742454447b93be8ab983887217db204371a77bd\n");

    let output = mush!(dir)
        .arg("cat-file")
        .arg("-p")
        .arg("0742454447b93be8ab983887217db204371a77bd")
        .output()
        .unwrap();

    assert_output_success(&output);
    assert_eq!("100644 blob acbe86c7c89586e0912a0a851bacf309c595c308\tx\n 40000 tree 892b8c36b1579b893c2eb05641d4361bd25ffde9\ty\n", String::from_utf8_lossy(&output.stdout));

    let output = mush!(dir)
        .arg("cat-file")
        .arg("-p")
        .arg("892b8c36b1579b893c2eb05641d4361bd25ffde9")
        .output()
        .unwrap();

    assert_output_success(&output);
    assert_eq!("100644 blob 8baef1b4abc478178b004d62031cf7fe6db6f903\txyz.txt\n", String::from_utf8_lossy(&output.stdout));
}
