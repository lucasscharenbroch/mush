mod helpers;

use std::os::unix::fs::MetadataExt;

use helpers::*;

#[test]
fn create_new_index() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);
    create_dir(dir.path(), "x");

    let filename = "x/999.txt";
    let contents = "pedal\nstroke\nmush\n999\n";
    let hash = "99622a960c9f3d0232df4d90149b666c11159b9e";

    let file = create_file_with_contents(dir.path(), filename, contents);
    let stat = file.metadata().unwrap();

    // Hard-coding this because inode, device and timestamps will change
    // per every run.
    // An alternative would be to use git and compare the two, but I
    // don't want to create the dependency, nor add the extra potential
    // for mismatch due to global config or future changes to git.
    // Not that this approach is bullet-proof either.
    // I verified that the below matches git on my machine when this testcase was written.
    let mut expected_index_contents: Vec<u8> =
        Vec::from([
            &[0x44, 0x49, 0x52, 0x43], // DIRC
            &[0x00, 0x00, 0x00, 0x02], // version 2
            &[0x00, 0x00, 0x00, 0x01], // 1 entry

            // first entry:
            &(stat.ctime() as u32).to_be_bytes()[..], // ctime seconds
            &(stat.ctime_nsec() as u32).to_be_bytes()[..], // ctime nanoseconds
            &(stat.mtime() as u32).to_be_bytes()[..], // mtime seconds
            &(stat.mtime_nsec() as u32).to_be_bytes()[..], // mtime nanoseconds
            &(stat.dev() as u32).to_be_bytes()[..], // dev
            &(stat.ino() as u32).to_be_bytes()[..], // ino
            &stat.mode().to_be_bytes()[..], // mode
            &stat.uid().to_be_bytes()[..], // uid
            &stat.gid().to_be_bytes()[..], // gid
            &[0x00, 0x00, 0x00, 0x16], // file size
            &[0x99, 0x62, 0x2a, 0x96, 0x0c,
             0x9f, 0x3d, 0x02, 0x32, 0xdf,
             0x4d, 0x90, 0x14, 0x9b, 0x66,
             0x6c, 0x11, 0x15, 0x9b, 0x9e], // hash (`hex::encode(hash)`)
            &[0x00, 0x09], // flags
            &[0x78, 0x2f, 0x39, 0x39, 0x39, 0x2e, 0x74, 0x78, 0x74, 0x00], // "x/999.txt\0"
            &[0x00, 0x00], // 0 extensions
        ].concat());

    // Add checksum. Unforfunately hard to sanity-check this one.
    expected_index_contents.extend(mush::hash::Hash::digest(&expected_index_contents).as_bytes());

    let output = mush!(dir)
            .arg("hash-object")
            .arg(filename)
            .output()
            .unwrap();

    assert_output_success(&output);
    assert_eq!(
        format!("{hash}\n"),
        String::from_utf8(output.stdout).unwrap()
    );

    let output = mush!(dir)
            .arg("update-index")
            .arg("--add")
            .arg(hash)
            .arg(filename)
            .output()
            .unwrap();

    assert_output_success(&output);

    assert_file_contents(&dir.path().join(".mush/index"), &expected_index_contents);
}

#[test]
fn add_to_existing_index() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    let existing_entry: &[u8] = &[
        0x67, 0x9d, 0x77, 0x9f, 0x12, 0xee, 0x03, 0xae, 0x67, 0x9d, 0x77, 0x9f,
        0x12, 0xee, 0x03, 0xae, 0x00, 0x01, 0x03, 0x07, 0x00, 0x3e, 0x4b, 0xab,
        0x00, 0x00, 0x81, 0xa4, 0x00, 0x00, 0x03, 0xe8, 0x00, 0x00, 0x03, 0xe6,
        0x00, 0x00, 0x00, 0xdb, 0xda, 0x9e, 0xfe, 0xd4, 0x6f, 0x06, 0x3d, 0x03,
        0x28, 0xde, 0x76, 0xca, 0x16, 0x07, 0x03, 0x78, 0xb9, 0xbb, 0x8e, 0xb3,
        0x00, 0x07, 0x67, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x73, 0x00,
    ];

    let initial_index_contents: Vec<u8> = [
        &[0x44, 0x49, 0x52, 0x43], // DIRC
        &[0x00, 0x00, 0x00, 0x02], // version 2
        &[0x00, 0x00, 0x00, 0x01], // 1 entry

        &existing_entry[..],

        &[0x00, 0x00, // 0 extensions
        0x39, 0x8b, 0x79, 0x6a, 0x01,
        0xf5, 0xbe, 0x8e, 0x33, 0x6c,
        0x38, 0xce, 0x14, 0xef, 0xc3,
        0x1b, 0x80, 0x77, 0x47, 0x55], // checksum
    ].concat();

    create_file_with_byte_contents(dir.path(), ".mush/index", initial_index_contents.as_slice());

    let filename = "999.txt";
    let contents = "pedal\nstroke\nmush\n999\n";
    let hash = "99622a960c9f3d0232df4d90149b666c11159b9e";

    let file = create_file_with_contents(dir.path(), filename, contents);
    let stat = file.metadata().unwrap();

    let mut expected_index_contents: Vec<u8> =
        Vec::from([
            &[0x44, 0x49, 0x52, 0x43], // DIRC
            &[0x00, 0x00, 0x00, 0x02], // version 2
            &[0x00, 0x00, 0x00, 0x02], // 2 entries

            // first entry:
            &(stat.ctime() as u32).to_be_bytes()[..], // ctime seconds
            &(stat.ctime_nsec() as u32).to_be_bytes()[..], // ctime nanoseconds
            &(stat.mtime() as u32).to_be_bytes()[..], // mtime seconds
            &(stat.mtime_nsec() as u32).to_be_bytes()[..], // mtime nanoseconds
            &(stat.dev() as u32).to_be_bytes()[..], // dev
            &(stat.ino() as u32).to_be_bytes()[..], // ino
            &stat.mode().to_be_bytes()[..], // mode
            &stat.uid().to_be_bytes()[..], // uid
            &stat.gid().to_be_bytes()[..], // gid
            &[0x00, 0x00, 0x00, 0x16], // file size
            &[0x99, 0x62, 0x2a, 0x96, 0x0c,
             0x9f, 0x3d, 0x02, 0x32, 0xdf,
             0x4d, 0x90, 0x14, 0x9b, 0x66,
             0x6c, 0x11, 0x15, 0x9b, 0x9e], // hash (`hex::encode(hash)`)
            &[0x00, 0x07], // flags
            &[0x39, 0x39, 0x39, 0x2e, 0x74, 0x78, 0x74, 0x00], // "999.txt\0"

            // existing entry:
            existing_entry,

            &[0x00, 0x00], // 0 extensions
        ].concat());

    // Add checksum. Unforfunately hard to sanity-check this one.
    expected_index_contents.extend(mush::hash::Hash::digest(&expected_index_contents).as_bytes());

    let output = mush!(dir)
            .arg("hash-object")
            .arg(filename)
            .output()
            .unwrap();

    assert_output_success(&output);
    assert_eq!(
        format!("{hash}\n"),
        String::from_utf8(output.stdout).unwrap()
    );

    let output = mush!(dir)
            .arg("update-index")
            .arg("--add")
            .arg(hash)
            .arg(filename)
            .output()
            .unwrap();

    assert_output_success(&output);

    assert_file_contents(&dir.path().join(".mush/index"), &expected_index_contents);
}

#[test]
fn remove_until_empty() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    let existing_entry_1: &[u8] = &[ // "999.txt"
        0x67, 0x9d, 0x76, 0xd4, 0x37, 0xc9, 0x70, 0x1c, 0x67, 0x9d, 0x76, 0xd4,
        0x37, 0xc9, 0x70, 0x1c, 0x00, 0x01, 0x03, 0x07, 0x00, 0x3e, 0x4b, 0xa4,
        0x00, 0x00, 0x81, 0xa4, 0x00, 0x00, 0x03, 0xe8, 0x00, 0x00, 0x03, 0xe6,
        0x00, 0x00, 0x00, 0x16, 0x99, 0x62, 0x2a, 0x96, 0x0c, 0x9f, 0x3d, 0x02,
        0x32, 0xdf, 0x4d, 0x90, 0x14, 0x9b, 0x66, 0x6c, 0x11, 0x15, 0x9b, 0x9e,
        0x00, 0x07, 0x39, 0x39, 0x39, 0x2e, 0x74, 0x78, 0x74, 0x00,
    ];

    let existing_entry_2: &[u8] = &[ // "goggins"
        0x67, 0x9d, 0x77, 0x9f, 0x12, 0xee, 0x03, 0xae, 0x67, 0x9d, 0x77, 0x9f,
        0x12, 0xee, 0x03, 0xae, 0x00, 0x01, 0x03, 0x07, 0x00, 0x3e, 0x4b, 0xab,
        0x00, 0x00, 0x81, 0xa4, 0x00, 0x00, 0x03, 0xe8, 0x00, 0x00, 0x03, 0xe6,
        0x00, 0x00, 0x00, 0xdb, 0xda, 0x9e, 0xfe, 0xd4, 0x6f, 0x06, 0x3d, 0x03,
        0x28, 0xde, 0x76, 0xca, 0x16, 0x07, 0x03, 0x78, 0xb9, 0xbb, 0x8e, 0xb3,
        0x00, 0x07, 0x67, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x73, 0x00,
    ];

    let initial_index_contents: Vec<u8> = [
        &[0x44, 0x49, 0x52, 0x43], // DIRC
        &[0x00, 0x00, 0x00, 0x02], // version 2
        &[0x00, 0x00, 0x00, 0x02], // 2 entries

        &existing_entry_1[..],
        &existing_entry_2[..],

        &[0x00, 0x00], // 0 extensions
        &[0x9b, 0xe1, 0x5c, 0x37, 0x1d,
        0x30, 0xf4, 0x2b, 0x27, 0x2b,
        0x5b, 0x10, 0xc1, 0x31, 0xc1,
        0xab, 0x4e, 0xe9, 0xae, 0x0f], // checksum
    ].concat();

    create_file_with_byte_contents(dir.path(), ".mush/index", initial_index_contents.as_slice());

    let after_first_removal: Vec<u8> = [
        &[0x44, 0x49, 0x52, 0x43], // DIRC
        &[0x00, 0x00, 0x00, 0x02], // version 2
        &[0x00, 0x00, 0x00, 0x01], // 1 entry

        &existing_entry_2[..],

        &[0x00, 0x00], // 0 extensions
        &[0x39, 0x8b, 0x79, 0x6a, 0x01,
        0xf5, 0xbe, 0x8e, 0x33, 0x6c,
        0x38, 0xce, 0x14, 0xef, 0xc3,
        0x1b, 0x80, 0x77, 0x47, 0x55], // checksum
    ].concat();

    let output = mush!(dir)
            .arg("update-index")
            .arg("--remove")
            .arg("./999.txt")
            .output()
            .unwrap();

    assert_output_success(&output);
    assert_file_contents(&dir.path().join(".mush/index"), &after_first_removal);

    let after_second_removal: Vec<u8> = [
        &[0x44, 0x49, 0x52, 0x43], // DIRC
        &[0x00, 0x00, 0x00, 0x02], // version 2
        &[0x00, 0x00, 0x00, 0x00], // 0 entries

        &[0x00, 0x00][..], // 0 extensions
        &[0x65, 0x61, 0xed, 0x98, 0xa9,
        0x85, 0x5a, 0x5f, 0x21, 0x36,
        0xd8, 0xf5, 0x3f, 0xa6, 0xa7,
        0xc6, 0x44, 0x4d, 0xfe, 0x82], // checksum
    ].concat();

    let output = mush!(dir)
            .arg("update-index")
            .arg("--remove")
            .arg("goggins")
            .output()
            .unwrap();

    assert_output_success(&output);
    assert_file_contents(&dir.path().join(".mush/index"), &after_second_removal);
}
