use std::io;

use hexyl::{BorderStyle, PrinterBuilder};

fn main() {
    let input = vec![
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x44, 0x08, 0x02, 0x00, 0x00, 0x00,
    ];

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let show_color = true;
    let show_char_panel = true;
    let show_position_panel = true;
    let use_squeezing = false;
    let border_style = BorderStyle::Unicode;
    let panels = 2;

    let mut printer = PrinterBuilder::new(&mut handle)
        .show_color(show_color)
        .show_char_panel(show_char_panel)
        .show_position_panel(show_position_panel)
        .with_border_style(border_style)
        .with_squeeze(use_squeezing)
        .with_panels(panels)
        .build();
    printer.print_all(&input[..]).unwrap();
}
