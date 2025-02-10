mod helpers;

use std::{io::Write, process::Stdio};

use helpers::*;

#[test]
fn set_and_read_several_options() {
    let dir = tempdir();
    mush_init_clean_repo(&dir);

    [
        ("d", "d", "another   "),
        ("a.b.c", "a/b/c", "some value"),
        ("e.f", "e/f", "pedal\nstroke\nmush\n403\n"),
    ].iter().for_each(|(option, path, value)| {
        let output = mush!(dir)
                .arg("config")
                .arg(option)
                .output()
                .unwrap();

		assert_eq!(false, output.status.success());

        let output = mush!(dir)
                .arg("config")
                .arg(option)
                .arg(value)
                .output()
                .unwrap();

        assert_output_success(&output);
		assert_file_contents(&dir.path().join(format!(".mush/config/{path}")), value);

        let output = mush!(dir)
                .arg("config")
                .arg(option)
                .output()
                .unwrap();

        assert_output_success(&output);
		assert_eq!(
			format!("{value}\n").as_bytes(),
			output.stdout
		);

		// update to "xyz"
        let output = mush!(dir)
                .arg("config")
                .arg(option)
                .arg("xyz")
                .output()
                .unwrap();

        assert_output_success(&output);
		assert_file_contents(&dir.path().join(format!(".mush/config/{path}")), &"xyz");
    });
}
