use std::io;
use hexyl::{themes, BorderStyle, InputFormat, Printer};

fn main() {
    let input = (0..=255).map(|v| v).collect::<Vec<u8>>();
    let stdin = io::stdout();

    let mut handle        = stdin.lock();
    let     theme         = Some(themes::Hexylamine);
    let     border_style  = BorderStyle::Unicode;
    let     input_format  = InputFormat::Ascii;
    let     use_squeezing = false;
    let     upper_case    = false;

    Printer::new (
        &mut handle,
        theme,
        border_style,
        input_format,
        use_squeezing,
        upper_case,
    )
    .print_all(&input[..])
    .unwrap();
}
