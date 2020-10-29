use assert_cmd::Command;

fn hexyl() -> Command {
    let mut cmd = Command::cargo_bin("hexyl").unwrap();
    cmd.current_dir("tests/examples");
    cmd
}

#[test]
fn can_print_simple_ascii_file() {
    hexyl()
        .arg("ascii")
        .arg("--color=never")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000000│ 61 62 63 64 65 66 67 68 ┊ 21 3f 25 26 2f 28 29 0a │abcdefgh┊!?%&/()_│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
}

#[test]
fn fails_on_non_existing_input() {
    hexyl().arg("non-existing").assert().failure();
}

#[test]
fn display_offset() {
    hexyl()
        .arg("ascii")
        .arg("--color=never")
        .arg("--display-offset=0xc0ffee")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00c0ffee│ 61 62 63 64 65 66 67 68 ┊ 21 3f 25 26 2f 28 29 0a │abcdefgh┊!?%&/()_│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
}
