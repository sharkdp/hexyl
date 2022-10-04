pub(crate) mod input;
pub mod squeezer;

pub use input::*;

use std::io::{self, Read, Write};

use ansi_term::Color;
use ansi_term::Color::Fixed;

use crate::squeezer::{SqueezeAction, Squeezer};

const BUFFER_SIZE: usize = 256;

const COLOR_NULL: Color = Fixed(242); // grey
const COLOR_OFFSET: Color = Fixed(242); // grey
const COLOR_ASCII_PRINTABLE: Color = Color::Cyan;
const COLOR_ASCII_WHITESPACE: Color = Color::Green;
const COLOR_ASCII_OTHER: Color = Color::Purple;
const COLOR_NONASCII: Color = Color::Yellow;

pub enum ByteCategory {
    Null,
    AsciiPrintable,
    AsciiWhitespace,
    AsciiOther,
    NonAscii,
}

#[derive(Copy, Clone)]
struct Byte(u8);

impl Byte {
    fn category(self) -> ByteCategory {
        if self.0 == 0x00 {
            ByteCategory::Null
        } else if self.0.is_ascii_graphic() {
            ByteCategory::AsciiPrintable
        } else if self.0.is_ascii_whitespace() {
            ByteCategory::AsciiWhitespace
        } else if self.0.is_ascii() {
            ByteCategory::AsciiOther
        } else {
            ByteCategory::NonAscii
        }
    }

    fn color(self) -> &'static Color {
        use crate::ByteCategory::*;

        match self.category() {
            Null => &COLOR_NULL,
            AsciiPrintable => &COLOR_ASCII_PRINTABLE,
            AsciiWhitespace => &COLOR_ASCII_WHITESPACE,
            AsciiOther => &COLOR_ASCII_OTHER,
            NonAscii => &COLOR_NONASCII,
        }
    }

    fn as_char(self) -> char {
        use crate::ByteCategory::*;

        match self.category() {
            Null => '0',
            AsciiPrintable => self.0 as char,
            AsciiWhitespace if self.0 == 0x20 => ' ',
            AsciiWhitespace => '_',
            AsciiOther => '•',
            NonAscii => '×',
        }
    }
}

struct BorderElements {
    left_corner: char,
    horizontal_line: char,
    column_separator: char,
    right_corner: char,
}

#[derive(Clone, Copy)]
pub enum BorderStyle {
    Unicode,
    Ascii,
    None,
}

impl BorderStyle {
    fn header_elems(&self) -> Option<BorderElements> {
        match self {
            BorderStyle::Unicode => Some(BorderElements {
                left_corner: '┌',
                horizontal_line: '─',
                column_separator: '┬',
                right_corner: '┐',
            }),
            BorderStyle::Ascii => Some(BorderElements {
                left_corner: '+',
                horizontal_line: '-',
                column_separator: '+',
                right_corner: '+',
            }),
            BorderStyle::None => None,
        }
    }

    fn footer_elems(&self) -> Option<BorderElements> {
        match self {
            BorderStyle::Unicode => Some(BorderElements {
                left_corner: '└',
                horizontal_line: '─',
                column_separator: '┴',
                right_corner: '┘',
            }),
            BorderStyle::Ascii => Some(BorderElements {
                left_corner: '+',
                horizontal_line: '-',
                column_separator: '+',
                right_corner: '+',
            }),
            BorderStyle::None => None,
        }
    }

    fn outer_sep(&self) -> char {
        match self {
            BorderStyle::Unicode => '│',
            BorderStyle::Ascii => '|',
            BorderStyle::None => ' ',
        }
    }

    fn inner_sep(&self) -> char {
        match self {
            BorderStyle::Unicode => '┊',
            BorderStyle::Ascii => '|',
            BorderStyle::None => ' ',
        }
    }
}

pub struct PrinterBuilder<'a, Writer: Write> {
    writer: &'a mut Writer,
    show_color: bool,
    show_char_panel: bool,
    show_position_panel: bool,
    border_style: BorderStyle,
    use_squeeze: bool,
    panels: u16,
}

impl<'a, Writer: Write> PrinterBuilder<'a, Writer> {
    pub fn new(writer: &'a mut Writer) -> Self {
        PrinterBuilder {
            writer,
            show_color: true,
            show_char_panel: true,
            show_position_panel: true,
            border_style: BorderStyle::Unicode,
            use_squeeze: true,
            panels: 2,
        }
    }

    pub fn show_color(mut self, show_color: bool) -> Self {
        self.show_color = show_color;
        self
    }

    pub fn show_char_panel(mut self, show_char_panel: bool) -> Self {
        self.show_char_panel = show_char_panel;
        self
    }

    pub fn show_position_panel(mut self, show_position_panel: bool) -> Self {
        self.show_position_panel = show_position_panel;
        self
    }

    pub fn with_border_style(mut self, border_style: BorderStyle) -> Self {
        self.border_style = border_style;
        self
    }

    pub fn with_squeeze(mut self, use_squeeze: bool) -> Self {
        self.use_squeeze = use_squeeze;
        self
    }

    pub fn with_panels(mut self, panels: u16) -> Self {
        self.panels = panels;
        self
    }

    pub fn build(self) -> Printer<'a, Writer> {
        Printer::new(
            self.writer,
            self.show_color,
            self.show_char_panel,
            self.show_position_panel,
            self.border_style,
            self.use_squeeze,
            self.panels,
        )
    }
}

pub struct Printer<'a, Writer: Write> {
    idx: u64,
    /// The raw bytes used as input for the current line.
    raw_line: Vec<u8>,
    /// The buffered line built with each byte, ready to print to writer.
    buffer_line: Vec<u8>,
    writer: &'a mut Writer,
    show_color: bool,
    show_char_panel: bool,
    show_position_panel: bool,
    border_style: BorderStyle,
    header_was_printed: bool,
    byte_hex_panel: Vec<String>,
    byte_char_panel: Vec<String>,
    squeezer: Squeezer,
    display_offset: u64,
    /// The number of panels to draw.
    panels: u16,
}

impl<'a, Writer: Write> Printer<'a, Writer> {
    fn new(
        writer: &'a mut Writer,
        show_color: bool,
        show_char_panel: bool,
        show_position_panel: bool,
        border_style: BorderStyle,
        use_squeeze: bool,
        panels: u16,
    ) -> Printer<'a, Writer> {
        Printer {
            idx: 1,
            raw_line: vec![],
            buffer_line: vec![],
            writer,
            show_color,
            show_char_panel,
            show_position_panel,
            border_style,
            header_was_printed: false,
            byte_hex_panel: (0u8..=u8::MAX)
                .map(|i| {
                    let byte_hex = format!("{:02x} ", i);
                    if show_color {
                        Byte(i).color().paint(byte_hex).to_string()
                    } else {
                        byte_hex
                    }
                })
                .collect(),
            byte_char_panel: show_char_panel
                .then(|| {
                    (0u8..=u8::MAX)
                        .map(|i| {
                            let byte_char = format!("{}", Byte(i).as_char());
                            if show_color {
                                Byte(i).color().paint(byte_char).to_string()
                            } else {
                                byte_char
                            }
                        })
                        .collect()
                })
                .unwrap_or_default(),
            squeezer: Squeezer::new(use_squeeze, 8 * panels as u64),
            display_offset: 0,
            panels,
        }
    }

    pub fn display_offset(&mut self, display_offset: u64) -> &mut Self {
        self.display_offset = display_offset;
        self
    }

    fn write_border(&mut self, border_elements: BorderElements) {
        let h = border_elements.horizontal_line;
        let c = border_elements.column_separator;
        let l = border_elements.left_corner;
        let r = border_elements.right_corner;
        let h8 = h.to_string().repeat(8);
        let h25 = h.to_string().repeat(25);

        if self.show_position_panel {
            write!(self.writer, "{l}{h8}{c}", l = l, c = c, h8 = h8).ok();
        } else {
            write!(self.writer, "{}", l).ok();
        }

        for _ in 0..self.panels - 1 {
            write!(self.writer, "{h25}{c}", h25 = h25, c = c).ok();
        }
        if self.show_char_panel {
            write!(self.writer, "{h25}{c}", h25 = h25, c = c).ok();
        } else {
            write!(self.writer, "{h25}", h25 = h25).ok();
        }

        if self.show_char_panel {
            for _ in 0..self.panels - 1 {
                write!(self.writer, "{h8}{c}", h8 = h8, c = c).ok();
            }
            writeln!(self.writer, "{h8}{r}", h8 = h8, r = r).ok();
        } else {
            writeln!(self.writer, "{r}", r = r).ok();
        }
    }

    pub fn print_header(&mut self) {
        if self.header_was_printed {
            return;
        }
        if let Some(e) = self.border_style.header_elems() {
            self.write_border(e)
        }
        self.header_was_printed = true;
    }

    pub fn print_footer(&mut self) {
        if let Some(e) = self.border_style.footer_elems() {
            self.write_border(e)
        }
    }

    fn print_position_panel(&mut self) {
        if !self.show_position_panel {
            write!(&mut self.buffer_line, "{} ", self.border_style.outer_sep()).ok();
            return;
        }

        let style = COLOR_OFFSET.normal();
        let byte_index = format!("{:08x}", self.idx - 1 + self.display_offset);
        let formatted_string = if self.show_color {
            format!("{}", style.paint(byte_index))
        } else {
            byte_index
        };
        let _ = write!(
            &mut self.buffer_line,
            "{}{}{} ",
            self.border_style.outer_sep(),
            formatted_string,
            self.border_style.outer_sep()
        );
    }

    pub fn print_char_panel(&mut self) {
        if !self.show_char_panel {
            // just write newline if character panel is hidden
            writeln!(&mut self.buffer_line).ok();
            return;
        }

        let len = self.raw_line.len();

        let mut idx = 1;
        for &b in self.raw_line.iter() {
            let _ = write!(
                &mut self.buffer_line,
                "{}",
                self.byte_char_panel[b as usize]
            );

            if idx % 8 == 0 && idx % (u64::from(self.panels) * 8) != 0 {
                let _ = write!(&mut self.buffer_line, "{}", self.border_style.inner_sep());
            }

            idx += 1;
        }

        if len < usize::from(8 * self.panels) {
            let _ = write!(&mut self.buffer_line, "{0:1$}", "", 8 - len % 8);
            for _ in 0..(usize::from(8 * self.panels) - (len + (8 - len % 8))) / 8 {
                let _ = write!(
                    &mut self.buffer_line,
                    "{2}{0:1$}",
                    "",
                    8,
                    self.border_style.inner_sep()
                );
            }
        }
        let _ = writeln!(&mut self.buffer_line, "{}", self.border_style.outer_sep());
    }

    pub fn print_byte(&mut self, b: u8) -> io::Result<()> {
        if self.idx % (u64::from(self.panels) * 8) == 1 {
            self.print_header();
            self.print_position_panel();
        }

        write!(&mut self.buffer_line, "{}", self.byte_hex_panel[b as usize])?;
        self.raw_line.push(b);

        self.squeezer.process(b, self.idx);

        if self.idx % (u64::from(self.panels) * 8) == 0 {
            self.print_textline()?;
        } else if self.idx % 8 == 0 {
            let _ = write!(&mut self.buffer_line, "{} ", self.border_style.inner_sep());
        }

        self.idx += 1;

        Ok(())
    }

    pub fn print_textline(&mut self) -> io::Result<()> {
        let len = self.raw_line.len();

        if len == 0 {
            if self.squeezer.active() {
                self.print_position_panel();
                write!(&mut self.buffer_line, "{0:1$}", "", 24)?;
                for _ in 0..self.panels - 1 {
                    write!(
                        &mut self.buffer_line,
                        "{2}{0:1$}",
                        "",
                        25,
                        self.border_style.inner_sep()
                    )?;
                }
                write!(
                    &mut self.buffer_line,
                    "{2}{0:1$}",
                    "",
                    8,
                    self.border_style.outer_sep()
                )?;
                for _ in 0..self.panels - 1 {
                    write!(
                        &mut self.buffer_line,
                        "{2}{0:1$}",
                        "",
                        8,
                        self.border_style.inner_sep()
                    )?;
                }
                writeln!(&mut self.buffer_line, "{}", self.border_style.outer_sep())?;
                self.writer.write_all(&self.buffer_line)?;
            }
            return Ok(());
        }

        let squeeze_action = self.squeezer.action();

        // print empty space on last line
        if squeeze_action != SqueezeAction::Delete {
            if len < usize::from(8 * self.panels) {
                write!(&mut self.buffer_line, "{0:1$}", "", 3 * (8 - len % 8))?;
                for _ in 0..(usize::from(8 * self.panels) - (len + (8 - len % 8))) / 8 {
                    write!(
                        &mut self.buffer_line,
                        "{2}{0:1$}",
                        "",
                        1 + 3 * 8,
                        self.border_style.inner_sep()
                    )?;
                }
            }
            write!(&mut self.buffer_line, "{}", self.border_style.outer_sep())?;
        }
        self.print_char_panel();

        match squeeze_action {
            SqueezeAction::Print => {
                self.buffer_line.clear();
                let style = COLOR_OFFSET.normal();
                let asterisk = if self.show_color {
                    format!("{}", style.paint("*"))
                } else {
                    String::from("*")
                };

                write!(
                    &mut self.buffer_line,
                    "{3}{0}{1:2$}{3}",
                    asterisk,
                    "",
                    7,
                    self.border_style.outer_sep()
                )?;

                for i in 0..self.panels {
                    write!(&mut self.buffer_line, "{0:1$}", "", 25)?;
                    if i != self.panels - 1 {
                        write!(&mut self.buffer_line, "{}", self.border_style.inner_sep())?;
                    } else {
                        write!(&mut self.buffer_line, "{}", self.border_style.outer_sep())?;
                    }
                }

                for i in 0..self.panels {
                    write!(&mut self.buffer_line, "{0:1$}", "", 8)?;
                    if i != self.panels - 1 {
                        write!(&mut self.buffer_line, "{}", self.border_style.inner_sep())?;
                    } else {
                        writeln!(&mut self.buffer_line, "{}", self.border_style.outer_sep())?;
                    }
                }
            }
            SqueezeAction::Delete => self.buffer_line.clear(),
            SqueezeAction::Ignore => (),
        }

        self.writer.write_all(&self.buffer_line)?;

        self.raw_line.clear();
        self.buffer_line.clear();

        self.squeezer.advance();

        Ok(())
    }

    pub fn header_was_printed(&self) -> bool {
        self.header_was_printed
    }

    /// Loop through the given `Reader`, printing until the `Reader` buffer
    /// is exhausted.
    pub fn print_all<Reader: Read>(
        &mut self,
        mut reader: Reader,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut buffer = [0; BUFFER_SIZE];
        'mainloop: loop {
            let size = reader.read(&mut buffer)?;
            if size == 0 {
                break;
            }

            for b in &buffer[..size] {
                let res = self.print_byte(*b);

                if res.is_err() {
                    // Broken pipe
                    break 'mainloop;
                }
            }
        }

        // Finish last line
        self.print_textline().ok();

        if !self.header_was_printed() {
            self.print_header();
            if self.show_position_panel {
                write!(self.writer, "{0:9}", "│").ok();
            }
            write!(
                self.writer,
                "{0:2}{1:24}{0}{0:>26}",
                "│", "No content to print"
            )
            .ok();
            if self.show_char_panel {
                write!(self.writer, "{0:>9}{0:>9}", "│").ok();
            }
            writeln!(self.writer).ok();
        }
        self.print_footer();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::str;

    use super::*;

    fn assert_print_all_output<Reader: Read>(input: Reader, expected_string: String) {
        let mut output = vec![];
        let mut printer = Printer::new(
            &mut output,
            false,
            true,
            true,
            BorderStyle::Unicode,
            true,
            2,
        );

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string,)
    }

    #[test]
    fn empty_file_passes() {
        let input = io::empty();
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│        │ No content to print     │                         │        │        │
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

    #[test]
    fn display_offset() {
        let input = io::Cursor::new(b"spamspamspamspamspam");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│deadbeef│ 73 70 61 6d 73 70 61 6d ┊ 73 70 61 6d 73 70 61 6d │spamspam┊spamspam│
│deadbeff│ 73 70 61 6d             ┊                         │spam    ┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();

        let mut output = vec![];
        let mut printer: Printer<Vec<u8>> = Printer::new(
            &mut output,
            false,
            true,
            true,
            BorderStyle::Unicode,
            true,
            2,
        );
        printer.display_offset(0xdeadbeef);

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }

    #[test]
    fn multiple_panels() {
        let input = io::Cursor::new(b"supercalifragilisticexpialidocioussupercalifragilisticexpialidocioussupercalifragilisticexpialidocious");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬─────────────────────────┬─────────────────────────┬────────┬────────┬────────┬────────┐
│00000000│ 73 75 70 65 72 63 61 6c ┊ 69 66 72 61 67 69 6c 69 ┊ 73 74 69 63 65 78 70 69 ┊ 61 6c 69 64 6f 63 69 6f │supercal┊ifragili┊sticexpi┊alidocio│
│00000020│ 75 73 73 75 70 65 72 63 ┊ 61 6c 69 66 72 61 67 69 ┊ 6c 69 73 74 69 63 65 78 ┊ 70 69 61 6c 69 64 6f 63 │ussuperc┊alifragi┊listicex┊pialidoc│
│00000040│ 69 6f 75 73 73 75 70 65 ┊ 72 63 61 6c 69 66 72 61 ┊ 67 69 6c 69 73 74 69 63 ┊ 65 78 70 69 61 6c 69 64 │ioussupe┊rcalifra┊gilistic┊expialid│
│00000060│ 6f 63 69 6f 75 73       ┊                         ┊                         ┊                         │ocious  ┊        ┊        ┊        │
└────────┴─────────────────────────┴─────────────────────────┴─────────────────────────┴─────────────────────────┴────────┴────────┴────────┴────────┘
"
        .to_owned();

        let mut output = vec![];
        let mut printer: Printer<Vec<u8>> = Printer::new(
            &mut output,
            false,
            true,
            true,
            BorderStyle::Unicode,
            true,
            4,
        );

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }
}
