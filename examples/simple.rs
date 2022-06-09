use std::io;

use hexyl::{BorderStyle, ColorMode, Printer};

fn main() {
    let input = vec![
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x44, 0x08, 0x02, 0x00, 0x00, 0x00,
    ];

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let color_mode = Some(ColorMode::Color8Bit);
    let show_char_panel = true;
    let show_position_panel = true;
    let use_squeezing = false;
    let border_style = BorderStyle::Unicode;

    let mut printer = Printer::new(
        &mut handle,
        color_mode,
        show_char_panel,
        show_position_panel,
        border_style,
        use_squeezing,
    );
    printer.print_all(&input[..]).unwrap();
}
