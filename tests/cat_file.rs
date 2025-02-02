mod helpers;

use helpers::*;

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
fn pretty() {
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
