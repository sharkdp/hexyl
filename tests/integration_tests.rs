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
             │00000000│ 7f 45 4c 46 02 01 01 00 ┊ 00 00 00 00 00 00 00 00 │•ELF•••⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│\n\
             │00000010│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │•⋄>⋄•⋄⋄⋄┊⋄•@⋄⋄⋄⋄⋄│\n\
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
             │00000030│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │•⋄>⋄•⋄⋄⋄┊⋄•@⋄⋄⋄⋄⋄│\n\
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

mod group_and_endianness {
    use super::hexyl;
    use super::PrettyAssert;

    #[test]
    fn group_2_bytes_be() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-size=2")
            .assert()
            .success()
            .stdout(
                "┌────────┬─────────────────────┬─────────────────────┬────────┬────────┐\n\
                 │00000000│ 3031 3233 3435 3637 ┊ 3839 6162 6364 650a │01234567┊89abcde_│\n\
                 └────────┴─────────────────────┴─────────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_2_bytes_le() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-size=2")
            .arg("--endianness=little")
            .assert()
            .success()
            .stdout(
                "┌────────┬─────────────────────┬─────────────────────┬────────┬────────┐\n\
                 │00000000│ 3130 3332 3534 3736 ┊ 3938 6261 6463 0a65 │01234567┊89abcde_│\n\
                 └────────┴─────────────────────┴─────────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_4_bytes_be() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-size=4")
            .assert()
            .success()
            .stdout(
                "┌────────┬───────────────────┬───────────────────┬────────┬────────┐\n\
                 │00000000│ 30313233 34353637 ┊ 38396162 6364650a │01234567┊89abcde_│\n\
                 └────────┴───────────────────┴───────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_4_bytes_le() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-size=4")
            .arg("--endianness=little")
            .assert()
            .success()
            .stdout(
                "┌────────┬───────────────────┬───────────────────┬────────┬────────┐\n\
                 │00000000│ 33323130 37363534 ┊ 62613938 0a656463 │01234567┊89abcde_│\n\
                 └────────┴───────────────────┴───────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_8_bytes_be() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-size=8")
            .assert()
            .success()
            .stdout(
                "┌────────┬──────────────────┬──────────────────┬────────┬────────┐\n\
                 │00000000│ 3031323334353637 ┊ 383961626364650a │01234567┊89abcde_│\n\
                 └────────┴──────────────────┴──────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_8_bytes_le() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--group-size=8")
            .arg("--endianness=little")
            .assert()
            .success()
            .stdout(
                "┌────────┬──────────────────┬──────────────────┬────────┬────────┐\n\
                 │00000000│ 3736353433323130 ┊ 0a65646362613938 │01234567┊89abcde_│\n\
                 └────────┴──────────────────┴──────────────────┴────────┴────────┘\n",
            );
    }

    #[test]
    fn group_size_plain() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--plain")
            .arg("--group-size=2")
            .assert()
            .success()
            .stdout("  3031 3233 3435 3637   3839 6162 6364 650a  \n");
    }

    #[test]
    fn group_size_fill_space() {
        hexyl()
            .arg("--color=never")
            .arg("--group-size=2")
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
    fn group_size_invalid() {
        hexyl()
            .arg("ascii")
            .arg("--color=never")
            .arg("--plain")
            .arg("--group-size=3")
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
│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│*                        ┊                         │        ┊        │
│ ba 0e 00 00 00 b9 00 20 ┊ 40 00 bb 01 00 00 00 b8 │×•⋄⋄⋄×⋄ ┊@⋄×•⋄⋄⋄×│
│ 04 00 00 00 cd 80 b8 01 ┊ 00 00 00 cd 80 00 00 00 │•⋄⋄⋄×××•┊⋄⋄⋄××⋄⋄⋄│
│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
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
│ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄│
│*                        │        │
│ ba 0e 00 00 00 b9 00 20 │×•⋄⋄⋄×⋄ │
│ 40 00 bb 01 00 00 00 b8 │@⋄×•⋄⋄⋄×│
│ 04 00 00 00 cd 80 b8 01 │•⋄⋄⋄×××•│
│ 00 00 00 cd 80 00 00 00 │⋄⋄⋄××⋄⋄⋄│
│ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄│
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
│00000400│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│*       │                         ┊                         ┊                         │        ┊        ┊        │
│00001000│ ba 0e 00 00 00 b9 00 20 ┊ 40 00 bb 01 00 00 00 b8 ┊ 04 00 00 00 cd 80 b8 01 │×•⋄⋄⋄×⋄ ┊@⋄×•⋄⋄⋄×┊•⋄⋄⋄×××•│
│00001018│ 00 00 00 cd 80 00 00 00 ┊ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄××⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│00001030│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│*       │                         ┊                         ┊                         │        ┊        ┊        │
│000013f0│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00             ┊                         │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄    ┊        │
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
                "  00110000 00110001 00110010 00110011 00110100 00110101 00110110 00110111  \n  \
                   00111000 00111001 01100001 01100010 01100011 01100100 01100101 00001010  \n",
            );
    }
}

mod character_table {
    use super::hexyl;
    use super::PrettyAssert;

    #[test]
    fn ascii() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--character-table=ascii")
            .assert()
            .success()
            .pretty_stdout(
                "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 7f 45 4c 46 02 01 01 00 ┊ 00 00 00 00 00 00 00 00 │.ELF....┊........│
│00000010│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │..>.....┊..@.....│
│00000020│ 40 00 00 00 00 00 00 00 ┊ 28 20 00 00 00 00 00 00 │@.......┊( ......│
│00000030│ 00 00 00 00 40 00 38 00 ┊ 03 00 40 00 04 00 03 00 │....@.8.┊..@.....│
│00000040│ 01 00 00 00 04 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│00000050│ 00 00 40 00 00 00 00 00 ┊ 00 00 40 00 00 00 00 00 │..@.....┊..@.....│
│00000060│ e8 00 00 00 00 00 00 00 ┊ e8 00 00 00 00 00 00 00 │........┊........│
│00000070│ 00 10 00 00 00 00 00 00 ┊ 01 00 00 00 05 00 00 00 │........┊........│
│00000080│ 00 10 00 00 00 00 00 00 ┊ 00 10 40 00 00 00 00 00 │........┊..@.....│
│00000090│ 00 10 40 00 00 00 00 00 ┊ 1d 00 00 00 00 00 00 00 │..@.....┊........│
│000000a0│ 1d 00 00 00 00 00 00 00 ┊ 00 10 00 00 00 00 00 00 │........┊........│
│000000b0│ 01 00 00 00 06 00 00 00 ┊ 00 20 00 00 00 00 00 00 │........┊. ......│
│000000c0│ 00 20 40 00 00 00 00 00 ┊ 00 20 40 00 00 00 00 00 │. @.....┊. @.....│
│000000d0│ 0e 00 00 00 00 00 00 00 ┊ 0e 00 00 00 00 00 00 00 │........┊........│
│000000e0│ 00 10 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│000000f0│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│*       │                         ┊                         │        ┊        │
│00001000│ ba 0e 00 00 00 b9 00 20 ┊ 40 00 bb 01 00 00 00 b8 │....... ┊@.......│
│00001010│ 04 00 00 00 cd 80 b8 01 ┊ 00 00 00 cd 80 00 00 00 │........┊........│
│00001020│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│*       │                         ┊                         │        ┊        │
│00002000│ 48 65 6c 6c 6f 2c 20 77 ┊ 6f 72 6c 64 21 0a 00 2e │Hello, w┊orld!...│
│00002010│ 73 68 73 74 72 74 61 62 ┊ 00 2e 74 65 78 74 00 2e │shstrtab┊..text..│
│00002020│ 64 61 74 61 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │data....┊........│
│00002030│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│*       │                         ┊                         │        ┊        │
│00002060│ 00 00 00 00 00 00 00 00 ┊ 0b 00 00 00 01 00 00 00 │........┊........│
│00002070│ 06 00 00 00 00 00 00 00 ┊ 00 10 40 00 00 00 00 00 │........┊..@.....│
│00002080│ 00 10 00 00 00 00 00 00 ┊ 1d 00 00 00 00 00 00 00 │........┊........│
│00002090│ 00 00 00 00 00 00 00 00 ┊ 10 00 00 00 00 00 00 00 │........┊........│
│000020a0│ 00 00 00 00 00 00 00 00 ┊ 11 00 00 00 01 00 00 00 │........┊........│
│000020b0│ 03 00 00 00 00 00 00 00 ┊ 00 20 40 00 00 00 00 00 │........┊. @.....│
│000020c0│ 00 20 00 00 00 00 00 00 ┊ 0e 00 00 00 00 00 00 00 │. ......┊........│
│000020d0│ 00 00 00 00 00 00 00 00 ┊ 04 00 00 00 00 00 00 00 │........┊........│
│000020e0│ 00 00 00 00 00 00 00 00 ┊ 01 00 00 00 03 00 00 00 │........┊........│
│000020f0│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│00002100│ 0e 20 00 00 00 00 00 00 ┊ 17 00 00 00 00 00 00 00 │. ......┊........│
│00002110│ 00 00 00 00 00 00 00 00 ┊ 01 00 00 00 00 00 00 00 │........┊........│
│00002120│ 00 00 00 00 00 00 00 00 ┊                         │........┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
",
            );
    }

    #[test]
    fn codepage_437() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--character-table=codepage-437")
            .assert()
            .success()
            .pretty_stdout(
                "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 7f 45 4c 46 02 01 01 00 ┊ 00 00 00 00 00 00 00 00 │⌂ELF☻☺☺⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│00000010│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │☻⋄>⋄☺⋄⋄⋄┊⋄►@⋄⋄⋄⋄⋄│
│00000020│ 40 00 00 00 00 00 00 00 ┊ 28 20 00 00 00 00 00 00 │@⋄⋄⋄⋄⋄⋄⋄┊( ⋄⋄⋄⋄⋄⋄│
│00000030│ 00 00 00 00 40 00 38 00 ┊ 03 00 40 00 04 00 03 00 │⋄⋄⋄⋄@⋄8⋄┊♥⋄@⋄♦⋄♥⋄│
│00000040│ 01 00 00 00 04 00 00 00 ┊ 00 00 00 00 00 00 00 00 │☺⋄⋄⋄♦⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│00000050│ 00 00 40 00 00 00 00 00 ┊ 00 00 40 00 00 00 00 00 │⋄⋄@⋄⋄⋄⋄⋄┊⋄⋄@⋄⋄⋄⋄⋄│
│00000060│ e8 00 00 00 00 00 00 00 ┊ e8 00 00 00 00 00 00 00 │Φ⋄⋄⋄⋄⋄⋄⋄┊Φ⋄⋄⋄⋄⋄⋄⋄│
│00000070│ 00 10 00 00 00 00 00 00 ┊ 01 00 00 00 05 00 00 00 │⋄►⋄⋄⋄⋄⋄⋄┊☺⋄⋄⋄♣⋄⋄⋄│
│00000080│ 00 10 00 00 00 00 00 00 ┊ 00 10 40 00 00 00 00 00 │⋄►⋄⋄⋄⋄⋄⋄┊⋄►@⋄⋄⋄⋄⋄│
│00000090│ 00 10 40 00 00 00 00 00 ┊ 1d 00 00 00 00 00 00 00 │⋄►@⋄⋄⋄⋄⋄┊↔⋄⋄⋄⋄⋄⋄⋄│
│000000a0│ 1d 00 00 00 00 00 00 00 ┊ 00 10 00 00 00 00 00 00 │↔⋄⋄⋄⋄⋄⋄⋄┊⋄►⋄⋄⋄⋄⋄⋄│
│000000b0│ 01 00 00 00 06 00 00 00 ┊ 00 20 00 00 00 00 00 00 │☺⋄⋄⋄♠⋄⋄⋄┊⋄ ⋄⋄⋄⋄⋄⋄│
│000000c0│ 00 20 40 00 00 00 00 00 ┊ 00 20 40 00 00 00 00 00 │⋄ @⋄⋄⋄⋄⋄┊⋄ @⋄⋄⋄⋄⋄│
│000000d0│ 0e 00 00 00 00 00 00 00 ┊ 0e 00 00 00 00 00 00 00 │♫⋄⋄⋄⋄⋄⋄⋄┊♫⋄⋄⋄⋄⋄⋄⋄│
│000000e0│ 00 10 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄►⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│000000f0│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│*       │                         ┊                         │        ┊        │
│00001000│ ba 0e 00 00 00 b9 00 20 ┊ 40 00 bb 01 00 00 00 b8 │║♫⋄⋄⋄╣⋄ ┊@⋄╗☺⋄⋄⋄╕│
│00001010│ 04 00 00 00 cd 80 b8 01 ┊ 00 00 00 cd 80 00 00 00 │♦⋄⋄⋄═Ç╕☺┊⋄⋄⋄═Ç⋄⋄⋄│
│00001020│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│*       │                         ┊                         │        ┊        │
│00002000│ 48 65 6c 6c 6f 2c 20 77 ┊ 6f 72 6c 64 21 0a 00 2e │Hello, w┊orld!◙⋄.│
│00002010│ 73 68 73 74 72 74 61 62 ┊ 00 2e 74 65 78 74 00 2e │shstrtab┊⋄.text⋄.│
│00002020│ 64 61 74 61 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │data⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│00002030│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│*       │                         ┊                         │        ┊        │
│00002060│ 00 00 00 00 00 00 00 00 ┊ 0b 00 00 00 01 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊♂⋄⋄⋄☺⋄⋄⋄│
│00002070│ 06 00 00 00 00 00 00 00 ┊ 00 10 40 00 00 00 00 00 │♠⋄⋄⋄⋄⋄⋄⋄┊⋄►@⋄⋄⋄⋄⋄│
│00002080│ 00 10 00 00 00 00 00 00 ┊ 1d 00 00 00 00 00 00 00 │⋄►⋄⋄⋄⋄⋄⋄┊↔⋄⋄⋄⋄⋄⋄⋄│
│00002090│ 00 00 00 00 00 00 00 00 ┊ 10 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊►⋄⋄⋄⋄⋄⋄⋄│
│000020a0│ 00 00 00 00 00 00 00 00 ┊ 11 00 00 00 01 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊◄⋄⋄⋄☺⋄⋄⋄│
│000020b0│ 03 00 00 00 00 00 00 00 ┊ 00 20 40 00 00 00 00 00 │♥⋄⋄⋄⋄⋄⋄⋄┊⋄ @⋄⋄⋄⋄⋄│
│000020c0│ 00 20 00 00 00 00 00 00 ┊ 0e 00 00 00 00 00 00 00 │⋄ ⋄⋄⋄⋄⋄⋄┊♫⋄⋄⋄⋄⋄⋄⋄│
│000020d0│ 00 00 00 00 00 00 00 00 ┊ 04 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊♦⋄⋄⋄⋄⋄⋄⋄│
│000020e0│ 00 00 00 00 00 00 00 00 ┊ 01 00 00 00 03 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊☺⋄⋄⋄♥⋄⋄⋄│
│000020f0│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│00002100│ 0e 20 00 00 00 00 00 00 ┊ 17 00 00 00 00 00 00 00 │♫ ⋄⋄⋄⋄⋄⋄┊↨⋄⋄⋄⋄⋄⋄⋄│
│00002110│ 00 00 00 00 00 00 00 00 ┊ 01 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊☺⋄⋄⋄⋄⋄⋄⋄│
│00002120│ 00 00 00 00 00 00 00 00 ┊                         │⋄⋄⋄⋄⋄⋄⋄⋄┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
",
            );
    }

    #[test]
    fn codepage_1047() {
        hexyl()
            .arg("hello_world_elf64")
            .arg("--color=never")
            .arg("--character-table=codepage-1047")
            .assert()
            .success()
            .pretty_stdout(
                "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 7f 45 4c 46 02 01 01 00 ┊ 00 00 00 00 00 00 00 00 │..<.....┊........│
│00000010│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │........┊.. .....│
│00000020│ 40 00 00 00 00 00 00 00 ┊ 28 20 00 00 00 00 00 00 │ .......┊........│
│00000030│ 00 00 00 00 40 00 38 00 ┊ 03 00 40 00 04 00 03 00 │.... ...┊.. .....│
│00000040│ 01 00 00 00 04 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│00000050│ 00 00 40 00 00 00 00 00 ┊ 00 00 40 00 00 00 00 00 │.. .....┊.. .....│
│00000060│ e8 00 00 00 00 00 00 00 ┊ e8 00 00 00 00 00 00 00 │Y.......┊Y.......│
│00000070│ 00 10 00 00 00 00 00 00 ┊ 01 00 00 00 05 00 00 00 │........┊........│
│00000080│ 00 10 00 00 00 00 00 00 ┊ 00 10 40 00 00 00 00 00 │........┊.. .....│
│00000090│ 00 10 40 00 00 00 00 00 ┊ 1d 00 00 00 00 00 00 00 │.. .....┊........│
│000000a0│ 1d 00 00 00 00 00 00 00 ┊ 00 10 00 00 00 00 00 00 │........┊........│
│000000b0│ 01 00 00 00 06 00 00 00 ┊ 00 20 00 00 00 00 00 00 │........┊........│
│000000c0│ 00 20 40 00 00 00 00 00 ┊ 00 20 40 00 00 00 00 00 │.. .....┊.. .....│
│000000d0│ 0e 00 00 00 00 00 00 00 ┊ 0e 00 00 00 00 00 00 00 │........┊........│
│000000e0│ 00 10 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│000000f0│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│*       │                         ┊                         │        ┊        │
│00001000│ ba 0e 00 00 00 b9 00 20 ┊ 40 00 bb 01 00 00 00 b8 │[.......┊ .].....│
│00001010│ 04 00 00 00 cd 80 b8 01 ┊ 00 00 00 cd 80 00 00 00 │........┊........│
│00001020│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│*       │                         ┊                         │        ┊        │
│00002000│ 48 65 6c 6c 6f 2c 20 77 ┊ 6f 72 6c 64 21 0a 00 2e │..%%?...┊?.%.....│
│00002010│ 73 68 73 74 72 74 61 62 ┊ 00 2e 74 65 78 74 00 2e │....../.┊........│
│00002020│ 64 61 74 61 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │././....┊........│
│00002030│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│*       │                         ┊                         │        ┊        │
│00002060│ 00 00 00 00 00 00 00 00 ┊ 0b 00 00 00 01 00 00 00 │........┊........│
│00002070│ 06 00 00 00 00 00 00 00 ┊ 00 10 40 00 00 00 00 00 │........┊.. .....│
│00002080│ 00 10 00 00 00 00 00 00 ┊ 1d 00 00 00 00 00 00 00 │........┊........│
│00002090│ 00 00 00 00 00 00 00 00 ┊ 10 00 00 00 00 00 00 00 │........┊........│
│000020a0│ 00 00 00 00 00 00 00 00 ┊ 11 00 00 00 01 00 00 00 │........┊........│
│000020b0│ 03 00 00 00 00 00 00 00 ┊ 00 20 40 00 00 00 00 00 │........┊.. .....│
│000020c0│ 00 20 00 00 00 00 00 00 ┊ 0e 00 00 00 00 00 00 00 │........┊........│
│000020d0│ 00 00 00 00 00 00 00 00 ┊ 04 00 00 00 00 00 00 00 │........┊........│
│000020e0│ 00 00 00 00 00 00 00 00 ┊ 01 00 00 00 03 00 00 00 │........┊........│
│000020f0│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │........┊........│
│00002100│ 0e 20 00 00 00 00 00 00 ┊ 17 00 00 00 00 00 00 00 │........┊........│
│00002110│ 00 00 00 00 00 00 00 00 ┊ 01 00 00 00 00 00 00 00 │........┊........│
│00002120│ 00 00 00 00 00 00 00 00 ┊                         │........┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
",
            );
    }
}

mod colors {
    use super::hexyl;
    use owo_colors::{colors, Color};
    use std::collections::HashMap;

    // This is a helper for testing color in output. Writing tests to expect
    // raw color codes is ugly and hard to look at. Loading expected output
    // from files works fine, but you end up with a lot of files for all the
    // tests, and you have to cross-reference the file with the test that uses
    // it. The files also suffer from the same problem of being hard to
    // visually inspect. Just catting the file to see the colorized output
    // loses the nuance of where exactly the color codes appear (before or
    // after spaces, for example), or whether there are redundant codes.
    //
    // So this ColorMap solves the problem neatly by having two inputs:
    // - the easy to read expected output in plain format without any colors
    // - a mapping with identical structure except some characters replaced
    //   with single character color codes.
    // This makes it easy to reference the output and expected colors side by
    // side, and provides fairly precise control over exactly where color codes
    // are expected (the only caveat being you can't have two color codes back
    // to back). ColorMap combines these into the actual expected output.
    //
    // The color mapping needs to be identical to the expected output, except
    // it has some chars replaced by color code stand ins. These are replaced
    // with the actual color codes by the colorize method. The '.' character
    // is also ignored (it doesn't need to match the input). This makes the
    // color map more readable and avoids input characters from conflicting
    // with color chars.
    struct ColorMap {
        text_map: &'static str,
        char_to_color: HashMap<char, &'static str>,
    }

    impl ColorMap {
        fn from(text_map: &'static str) -> Self {
            ColorMap {
                text_map,
                char_to_color: HashMap::new(),
            }
        }

        fn with<C: Color>(&mut self, c: char) -> &mut Self {
            self.char_to_color.insert(c, C::ANSI_FG);
            self
        }

        fn colorize(&self, input: &str) -> String {
            let mut output = String::new();
            let mut input_chars = input.chars();
            for c in self.text_map.chars() {
                let next_input = input_chars.next().expect("input and color map don't match");
                if let Some(color) = self.char_to_color.get(&c) {
                    output.push_str(color);
                } else if c != '.' {
                    // ignore '.' in the mapping for readability
                    assert_eq!(c, next_input, "input and color map don't match");
                }
                output.push(next_input);
            }
            output
        }
    }

    #[test]
    fn hex_colors() {
        let input = b"He\x11\0 \xff\0\xdd";
        let expected_text = "\
            ┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
            │00000000│ 48 65 11 00 20 ff 00 dd ┊                         │He•⋄ ×⋄×┊        │\n\
            └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n";
        let expected = ColorMap::from(
            "\
            ┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
            │r.......d y. .. b. c. g. m. c. m.d┊                        d│y.bcgmcmd        d\n\
            └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        )
        .with::<colors::Red>('r')
        .with::<colors::Default>('d')
        .with::<colors::Yellow>('y')
        .with::<colors::Blue>('b')
        .with::<colors::Green>('g')
        .with::<colors::BrightMagenta>('m')
        .with::<colors::CustomColor<0xab, 0xcd, 0xef>>('c')
        .colorize(expected_text);

        hexyl()
            .write_stdin(input)
            .arg("--color=always")
            .env("HEXYL_OFFSET", "red")
            .env("HEXYL_ASCII_PRINTABLE", "yellow")
            .env("HEXYL_ASCII_WHITESPACE", "green")
            .env("HEXYL_ASCII_OTHER", "blue")
            .env("HEXYL_NONASCII", "bright magenta")
            .env("HEXYL_NULL", "#abcdef")
            .assert()
            .success()
            .stdout(expected);
    }

    #[test]
    fn binary_colors() {
        let input = b"He\x11\0 \xff\0\xdd";
        let expected_text = "\
            ┌────────┬─────────────────────────────────────────────────────────────────────────┬────────┐\n\
            │00000000│ 01001000 01100101 00010001 00000000 00100000 11111111 00000000 11011101 │He•⋄ ×⋄×│\n\
            └────────┴─────────────────────────────────────────────────────────────────────────┴────────┘\n";
        let expected = ColorMap::from(
            "\
            ┌────────┬─────────────────────────────────────────────────────────────────────────┬────────┐\n\
            │r.......d y....... ........ b....... c....... g....... m....... c....... m.......d│y.bcgmcmd\n\
            └────────┴─────────────────────────────────────────────────────────────────────────┴────────┘\n"
        )
        .with::<colors::Red>('r')
        .with::<colors::Default>('d')
        .with::<colors::Yellow>('y')
        .with::<colors::Blue>('b')
        .with::<colors::Green>('g')
        .with::<colors::BrightMagenta>('m')
        .with::<colors::CustomColor<0xab, 0xcd, 0xef>>('c')
        .colorize(expected_text);

        hexyl()
            .write_stdin(input)
            .arg("--color=always")
            .arg("--panels=1")
            .arg("--base=binary")
            .env("HEXYL_OFFSET", "red")
            .env("HEXYL_ASCII_PRINTABLE", "yellow")
            .env("HEXYL_ASCII_WHITESPACE", "green")
            .env("HEXYL_ASCII_OTHER", "blue")
            .env("HEXYL_NONASCII", "bright magenta")
            .env("HEXYL_NULL", "#abcdef")
            .assert()
            .success()
            .stdout(expected);
    }

    #[test]
    fn groupsize_colors() {
        let input = b"He\x11\0 \xff\0\xdd";
        let expected_text = "\
            ┌────────┬─────────────────────┬────────┐\n\
            │00000000│ 4865 1100 20ff 00dd │He•⋄ ×⋄×│\n\
            └────────┴─────────────────────┴────────┘\n";
        let expected = ColorMap::from(
            "\
            ┌────────┬─────────────────────┬────────┐\n\
            │r.......d y... b.c. g.m. c.m.d│y.bcgmcmd\n\
            └────────┴─────────────────────┴────────┘\n",
        )
        .with::<colors::Red>('r')
        .with::<colors::Default>('d')
        .with::<colors::Yellow>('y')
        .with::<colors::Blue>('b')
        .with::<colors::Green>('g')
        .with::<colors::BrightMagenta>('m')
        .with::<colors::CustomColor<0xab, 0xcd, 0xef>>('c')
        .colorize(expected_text);

        hexyl()
            .write_stdin(input)
            .arg("--color=always")
            .arg("--panels=1")
            .arg("--groupsize=2")
            .env("HEXYL_OFFSET", "red")
            .env("HEXYL_ASCII_PRINTABLE", "yellow")
            .env("HEXYL_ASCII_WHITESPACE", "green")
            .env("HEXYL_ASCII_OTHER", "blue")
            .env("HEXYL_NONASCII", "bright magenta")
            .env("HEXYL_NULL", "#abcdef")
            .assert()
            .success()
            .stdout(expected);
    }
}
