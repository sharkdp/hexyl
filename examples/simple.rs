use std::io;

use hexyl::{BorderStyle, Printer, themes};

fn main() {
    let input = vec![
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x44, 0x08, 0x02, 0x00, 0x00, 0x00,
    ];

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let theme = Some(themes::Hexylamine);
    let use_squeezing = false;
    let border_style = BorderStyle::Unicode;
    let input_format  = InputFormat::Ascii;

    let mut printer = Printer::new(&mut handle, theme, border_style, input_format, use_squeezing);
    printer.print_all(&input[..]).unwrap();
}
