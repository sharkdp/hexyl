use assert_cmd::Command;

fn hexyl() -> Command {
    let mut cmd = Command::cargo_bin("hexyl").unwrap();
    cmd.current_dir("tests/examples");
    cmd
}
trait PrettyAssert<S>
where
    S: AsRef<str>,
{
    fn pretty_stdout(self, other: S);
}

// https://github.com/assert-rs/assert_cmd/issues/121#issuecomment-849937376
//
impl<S> PrettyAssert<S> for assert_cmd::assert::Assert
where
    S: AsRef<str>,
{
    fn pretty_stdout(self, other: S) {
        println!("{}", other.as_ref().len());
        let self_str = String::from_utf8(self.get_output().stdout.clone()).unwrap();
        println!("{}", self_str.len());
        pretty_assertions::assert_eq!(self_str, other.as_ref());
    }
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
             │        │ No content              │                         │        │        │\n\
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
             │        │ No content              │                         │        │        │\n\
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

mod group {
    use super::hexyl;
    use super::PrettyAssert;

    #[test]
    fn group_2_bytes() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-bytes=2")
            .assert()
            .success()
            .stdout(
                "┌────────┬─────────────────────┬─────────────────────┬────────┬────────┐\n\
                 │00000000│ 3031 3233 3435 3637 ┊ 3839 6162 6364 650a │01234567┊89abcde_│\n\
                 └────────┴─────────────────────┴─────────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_4_bytes() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-bytes=4")
            .assert()
            .success()
            .stdout(
                "┌────────┬───────────────────┬───────────────────┬────────┬────────┐\n\
                 │00000000│ 30313233 34353637 ┊ 38396162 6364650a │01234567┊89abcde_│\n\
                 └────────┴───────────────────┴───────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_8_bytes() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-bytes=8")
            .assert()
            .success()
            .stdout(
                "┌────────┬──────────────────┬──────────────────┬────────┬────────┐\n\
                 │00000000│ 3031323334353637 ┊ 383961626364650a │01234567┊89abcde_│\n\
                 └────────┴──────────────────┴──────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_bytes_plain() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--plain")
            .arg("--group-bytes=2")
            .assert()
            .success()
            .stdout("  3031 3233 3435 3637   3839 6162 6364 650a  \n");
    }

    #[test]
    fn group_bytes_fill_space() {
        hexyl()
            .arg("--color=never")
            .arg("--group-bytes=2")
            .write_stdin("abc")
            .assert()
            .success()
            .stdout(
                "┌────────┬─────────────────────┬─────────────────────┬────────┬────────┐\n\
                 │00000000│ 6162 63             ┊                     │abc     ┊        │\n\
                 └────────┴─────────────────────┴─────────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_bytes_invalid() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--plain")
            .arg("--group-bytes=3")
            .assert()
            .failure();
    }
    #[test]
    fn squeeze_no_chars() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--skip=1024")
            .arg("--length=4096")
            .arg("--no-characters")
            .assert()
            .success()
            .pretty_stdout(
                "\
┌────────┬─────────────────────────┬─────────────────────────┐
│00000400│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │
│*       │                         ┊                         │
│00001000│ ba 0e 00 00 00 b9 00 20 ┊ 40 00 bb 01 00 00 00 b8 │
│00001010│ 04 00 00 00 cd 80 b8 01 ┊ 00 00 00 cd 80 00 00 00 │
│00001020│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │
│*       │                         ┊                         │
│00001400│                         ┊                         │
└────────┴─────────────────────────┴─────────────────────────┘
",
            );
    }
    #[test]
    fn squeeze_no_chars_one_panel() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--skip=1024")
            .arg("--length=4096")
            .arg("--no-characters")
            .arg("--panels=1")
            .assert()
            .success()
            .pretty_stdout(
                "\
┌────────┬─────────────────────────┐
│00000400│ 00 00 00 00 00 00 00 00 │
│*       │                         │
│00001000│ ba 0e 00 00 00 b9 00 20 │
│00001008│ 40 00 bb 01 00 00 00 b8 │
│00001010│ 04 00 00 00 cd 80 b8 01 │
│00001018│ 00 00 00 cd 80 00 00 00 │
│00001020│ 00 00 00 00 00 00 00 00 │
│*       │                         │
│00001400│                         │
└────────┴─────────────────────────┘
",
            );
    }
    #[test]
    fn squeeze_no_position() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--skip=1024")
            .arg("--length=4096")
            .arg("--no-position")
            .assert()
            .success()
            .pretty_stdout(
                "\
┌─────────────────────────┬─────────────────────────┬────────┬────────┐
│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │00000000┊00000000│
│*                        ┊                         │        ┊        │
│ ba 0e 00 00 00 b9 00 20 ┊ 40 00 bb 01 00 00 00 b8 │×•000×0 ┊@0×•000×│
│ 04 00 00 00 cd 80 b8 01 ┊ 00 00 00 cd 80 00 00 00 │•000×××•┊000××000│
│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │00000000┊00000000│
│*                        ┊                         │        ┊        │
│*                        ┊                         │        ┊        │
└─────────────────────────┴─────────────────────────┴────────┴────────┘
",
            );
    }
    #[test]
    fn squeeze_no_position_one_panel() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--skip=1024")
            .arg("--length=4096")
            .arg("--no-position")
            .arg("--panels=1")
            .assert()
            .success()
            .pretty_stdout(
                "\
┌─────────────────────────┬────────┐
│ 00 00 00 00 00 00 00 00 │00000000│
│*                        │        │
│ ba 0e 00 00 00 b9 00 20 │×•000×0 │
│ 40 00 bb 01 00 00 00 b8 │@0×•000×│
│ 04 00 00 00 cd 80 b8 01 │•000×××•│
│ 00 00 00 cd 80 00 00 00 │000××000│
│ 00 00 00 00 00 00 00 00 │00000000│
│*                        │        │
│*                        │        │
└─────────────────────────┴────────┘
",
            );
    }
    #[test]
    fn squeeze_odd_panels_remainder_bytes() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--skip=1024")
            .arg("--length=4092") // 4 byte remainder
            .arg("--panels=3")
            .assert()
            .success()
            .pretty_stdout(
                "\
┌────────┬─────────────────────────┬─────────────────────────┬─────────────────────────┬────────┬────────┬────────┐
│00000400│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │00000000┊00000000┊00000000│
│*       │                         ┊                         ┊                         │        ┊        ┊        │
│00001000│ ba 0e 00 00 00 b9 00 20 ┊ 40 00 bb 01 00 00 00 b8 ┊ 04 00 00 00 cd 80 b8 01 │×•000×0 ┊@0×•000×┊•000×××•│
│00001018│ 00 00 00 cd 80 00 00 00 ┊ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │000××000┊00000000┊00000000│
│00001030│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │00000000┊00000000┊00000000│
│*       │                         ┊                         ┊                         │        ┊        ┊        │
│000013f0│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00             ┊                         │00000000┊0000    ┊        │
└────────┴─────────────────────────┴─────────────────────────┴─────────────────────────┴────────┴────────┴────────┘
",
            );
    }

    #[test]
    fn squeeze_plain() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--skip=1024")
            .arg("--length=4096")
            .arg("--plain")
            .assert()
            .success()
            .pretty_stdout(
                "  \
  00 00 00 00 00 00 00 00   00 00 00 00 00 00 00 00  
 *                                                   
  ba 0e 00 00 00 b9 00 20   40 00 bb 01 00 00 00 b8  
  04 00 00 00 cd 80 b8 01   00 00 00 cd 80 00 00 00  
  00 00 00 00 00 00 00 00   00 00 00 00 00 00 00 00  
 *                                                   
 *                                                   
",
            );
    }

    #[test]
    fn squeeze_plain_remainder() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--skip=1024")
            .arg("--length=4092") // 4 byte remainder
            .arg("--plain")
            .assert()
            .success()
            .pretty_stdout(
                "  \
  00 00 00 00 00 00 00 00   00 00 00 00 00 00 00 00  
 *                                                   
  ba 0e 00 00 00 b9 00 20   40 00 bb 01 00 00 00 b8  
  04 00 00 00 cd 80 b8 01   00 00 00 cd 80 00 00 00  
  00 00 00 00 00 00 00 00   00 00 00 00 00 00 00 00  
 *                                                   
  00 00 00 00 00 00 00 00   00 00 00 00              
",
            );
    }
}

mod base {
    use super::hexyl;
    use super::PrettyAssert;

    #[test]
    fn base2() {
        hexyl()
            .arg("ascii")
            .arg("--plain")
            .arg("--base=binary")
            .assert()
            .success()
            .pretty_stdout(
                "  00110000 00110001 00110010 00110011 00110100 00110101 00110110 00110111   00111000 00111001 01100001 01100010 01100011 01100100 01100101 00001010  \n"
            );
    }
}