use std::io;

use hexyl::{BorderStyle, PrinterBuilder};

fn main() {
    let input = vec![
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x44, 0x08, 0x02, 0x00, 0x00, 0x00,
    ];

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut printer = PrinterBuilder::new(&mut handle)
        .show_color(true)
        .show_char_panel(true)
        .show_position_panel(true)
        .with_border_style(BorderStyle::Unicode)
        .enable_squeezing(false)
        .with_panels(2)
        .build();
    printer.print_all(&input[..]).unwrap();
}
