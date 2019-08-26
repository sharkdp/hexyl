use std::io;
use std::str;

use hexyl::{BorderStyle, Printer};

fn assert_print_all_output<Reader: io::Read>(input: Reader, expected_string: String) -> () {
    let mut output = vec![];
    let mut printer = Printer::new(&mut output, false, BorderStyle::Unicode, true);

    printer.print_all(input, None).unwrap();

    let actual_string: &str = str::from_utf8(&output).unwrap();
    assert_eq!(actual_string, expected_string)
}

#[test]
fn empty_file_passes() {
    let input = io::empty();
    let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
    .to_owned();
    assert_print_all_output(input, expected_string);
}

#[test]
fn short_input_passes() {
    let input = io::Cursor::new(b"spam");
    let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 73 70 61 6d             ┊                         │spam    ┊        │ 
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
    .to_owned();
    assert_print_all_output(input, expected_string);
}
