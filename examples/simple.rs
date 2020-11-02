use std::io;
use hexyl::{themes, BorderStyle, InputFormat, Printer};

fn main() {
    let input = vec![
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x44, 0x08, 0x02, 0x00, 0x00, 0x00,
    ];
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
