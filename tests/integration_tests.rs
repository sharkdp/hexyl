use assert_cmd::Command;

fn hexyl() -> Command {
    let mut cmd = Command::new(assert_cmd::cargo_bin!("hexyl"));
    cmd.current_dir("tests/examples");
    cmd
}

mod basic {
    use super::hexyl;

    #[test]
    fn can_print_simple_ascii_file() {
        hexyl()
        .arg("ascii")
        .arg("--color=never")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000000│ 30 31 32 33 34 35 36 37 ┊ 38 39 61 62 63 64 65 0a │01234567┊89abcde_│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }

    #[test]
    fn can_read_input_from_stdin() {
        hexyl()
        .arg("--color=never")
        .write_stdin("abc")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000000│ 61 62 63                ┊                         │abc     ┊        │\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }

    #[test]
    fn fails_on_non_existing_input() {
        hexyl().arg("non-existing").assert().failure();
    }

    #[test]
    fn prints_warning_on_empty_content() {
        hexyl()
        .arg("empty")
        .arg("--color=never")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │        │ No content to print     │                         │        │        │\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }
}

mod length {
    use super::hexyl;

    #[test]
    fn length_restricts_output_size() {
        hexyl()
        .arg("hello_world_elf64")
        .arg("--color=never")
        .arg("--length=32")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000000│ 7f 45 4c 46 02 01 01 00 ┊ 00 00 00 00 00 00 00 00 │•ELF•••0┊00000000│\n\
             │00000010│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │•0>0•000┊0•@00000│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }

    #[test]
    fn fail_if_length_and_bytes_options_are_used_simultaneously() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--length=32")
            .arg("--bytes=10")
            .assert()
            .failure();
    }

    #[test]
    fn fail_if_length_and_count_options_are_used_simultaneously() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--length=32")
            .arg("-l=10")
            .assert()
            .failure();
    }
}

mod bytes {
    use super::hexyl;

    #[test]
    fn fail_if_bytes_and_count_options_are_used_simultaneously() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--bytes=32")
            .arg("-l=10")
            .assert()
            .failure();
    }
}

mod skip {
    use super::hexyl;

    #[test]
    fn basic() {
        hexyl()
        .arg("ascii")
        .arg("--color=never")
        .arg("--skip=2")
        .arg("--length=4")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000002│ 32 33 34 35             ┊                         │2345    ┊        │\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }

    #[test]
    fn prints_warning_when_skipping_past_the_end() {
        hexyl()
        .arg("ascii")
        .arg("--color=never")
        .arg("--skip=1000")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │        │ No content to print     │                         │        │        │\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }

    #[test]
    fn negative_offset() {
        hexyl()
        .arg("ascii")
        .arg("--color=never")
        .arg("--skip=-4")
        .arg("--length=3")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │0000000c│ 63 64 65                ┊                         │cde     ┊        │\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }

    #[test]
    fn fails_if_negative_offset_is_too_large() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--skip=-1MiB")
            .assert()
            .failure()
            .stderr(predicates::str::contains("Failed to jump"));
    }
}

mod display_offset {
    use super::hexyl;

    #[test]
    fn basic() {
        hexyl()
        .arg("ascii")
        .arg("--color=never")
        .arg("--display-offset=0xc0ffee")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00c0ffee│ 30 31 32 33 34 35 36 37 ┊ 38 39 61 62 63 64 65 0a │01234567┊89abcde_│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }

    #[test]
    fn display_offset_and_skip() {
        hexyl()
        .arg("hello_world_elf64")
        .arg("--color=never")
        .arg("--display-offset=0x20")
        .arg("--skip=0x10")
        .arg("--length=0x10")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000030│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │•0>0•000┊0•@00000│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
    }
}

mod blocksize {
    use super::hexyl;

    #[test]
    fn fails_for_zero_or_negative_blocksize() {
        hexyl()
            .arg("ascii")
            .arg("--block-size=0")
            .assert()
            .failure();

        hexyl()
            .arg("ascii")
            .arg("--block-size=-16")
            .assert()
            .failure();
    }
}

mod display_settings {
    use super::hexyl;

    #[test]
    fn plain() {
        hexyl()
            .arg("ascii")
            .arg("--plain")
            .assert()
            .success()
            .stdout("  30 31 32 33 34 35 36 37   38 39 61 62 63 64 65 0a  \n");
    }

    #[test]
    fn no_chars() {
        hexyl()
            .arg("ascii")
            .arg("--no-characters")
            .arg("--color=never")
            .assert()
            .success()
            .stdout(
                "┌────────┬─────────────────────────┬─────────────────────────┐\n\
                 │00000000│ 30 31 32 33 34 35 36 37 ┊ 38 39 61 62 63 64 65 0a │\n\
                 └────────┴─────────────────────────┴─────────────────────────┘\n",
            );
    }

    #[test]
    fn no_position() {
        hexyl()
            .arg("ascii")
            .arg("--no-position")
            .arg("--color=never")
            .assert()
            .success()
            .stdout(
                "┌─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
                 │ 30 31 32 33 34 35 36 37 ┊ 38 39 61 62 63 64 65 0a │01234567┊89abcde_│\n\
                 └─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
            );
    }
}
