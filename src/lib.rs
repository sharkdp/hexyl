pub(crate) mod input;
pub mod squeezer;

pub use input::*;

use std::io::{self, BufReader, Read, Write};

use ansi_term::Color;
use ansi_term::Color::Fixed;

use crate::squeezer::{SqueezeAction, Squeezer};

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

    pub fn enable_squeezing(mut self, enable: bool) -> Self {
        self.use_squeeze = enable;
        self
    }

    pub fn num_panels(mut self, num: u16) -> Self {
        self.panels = num;
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
    /// the buffer containing all the bytes in a line for character printing
    line_buf: Vec<u8>,
    writer: &'a mut Writer,
    show_char_panel: bool,
    show_position_panel: bool,
    border_style: BorderStyle,
    byte_hex_panel: Vec<String>,
    byte_char_panel: Vec<String>,
    byte_hex_panel_g: Vec<String>,
    byte_char_panel_g: Vec<String>,
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
            idx: 0,
            line_buf: vec![],
            writer,
            show_char_panel,
            show_position_panel,
            border_style,
            byte_hex_panel: (0u8..=u8::MAX)
                .map(|i| {
                    let byte_hex = format!(" {:02x}", i);
                    if show_color {
                        Byte(i).color().paint(byte_hex).to_string()
                    } else {
                        byte_hex
                    }
                })
                .collect(),
            byte_char_panel: (0u8..=u8::MAX)
                .map(|i| {
                    let byte_char = format!("{}", Byte(i).as_char());
                    if show_color {
                        Byte(i).color().paint(byte_char).to_string()
                    } else {
                        byte_char
                    }
                })
                .collect(),
            byte_hex_panel_g: (0u8..=u8::MAX)
                .map(|i| {
                    let byte_hex = format!("{:02x}", i);
                    let style = COLOR_OFFSET.normal();
                    if show_color {
                        style.paint(byte_hex).to_string()
                    } else {
                        byte_hex
                    }
                })
                .collect(),
            byte_char_panel_g: (0u8..=u8::MAX)
                .map(|i| {
                    let byte_char = format!("{}", Byte(i).as_char());
                    let style = COLOR_OFFSET.normal();
                    if show_color {
                        style.paint(byte_char).to_string()
                    } else {
                        byte_char
                    }
                })
                .collect(),
            squeezer: Squeezer::new(use_squeeze, 8 * panels as u64),
            display_offset: 0,
            panels,
        }
    }

    pub fn display_offset(&mut self, display_offset: u64) -> &mut Self {
        self.display_offset = display_offset;
        self
    }

    fn write_border(&mut self, border_elements: BorderElements) -> io::Result<()> {
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

        Ok(())
    }

    pub fn print_header(&mut self) -> io::Result<()> {
        if let Some(e) = self.border_style.header_elems() {
            self.write_border(e)?
        }
        Ok(())
    }

    pub fn print_footer(&mut self) -> io::Result<()> {
        if let Some(e) = self.border_style.footer_elems() {
            self.write_border(e)?
        }
        Ok(())
    }

    fn print_position_panel(&mut self) -> io::Result<()> {
        match self.squeezer.action() {
            SqueezeAction::Print => {
                self.writer
                    .write_all(self.byte_char_panel_g[b'*' as usize].as_bytes())?;
                self.writer.write_all(b"       ")?;
                self.writer.write_all(
                    self.border_style
                        .outer_sep()
                        .encode_utf8(&mut [0; 4])
                        .as_bytes(),
                )?;
                Ok(())
            }
            SqueezeAction::Ignore => {
                let byte_index: [u8; 8] = (self.idx + self.display_offset).to_be_bytes();
                let mut i = 0;
                while byte_index[i] == 0x0 && i < 4 {
                    i += 1;
                }
                for &byte in byte_index.iter().skip(i) {
                    self.writer
                        .write_all(self.byte_hex_panel_g[byte as usize].as_bytes())?;
                }
                self.writer.write_all(
                    self.border_style
                        .outer_sep()
                        .encode_utf8(&mut [0; 4])
                        .as_bytes(),
                )?;
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    pub fn print_char_panel(&mut self) -> io::Result<()> {
        match self.squeezer.action() {
            SqueezeAction::Print => {
                for i in 0..self.panels {
                    self.writer.write_all(b"        ")?;
                    if i == self.panels - 1 {
                        self.writer.write_all(
                            self.border_style
                                .outer_sep()
                                .encode_utf8(&mut [0; 4])
                                .as_bytes(),
                        )?;
                        self.writer.write_all(b"\n")?;
                    } else {
                        self.writer.write_all(
                            self.border_style
                                .inner_sep()
                                .encode_utf8(&mut [0; 4])
                                .as_bytes(),
                        )?;
                    }
                }
            }
            SqueezeAction::Ignore => {
                let mut idx = 0;
                for &b in self.line_buf.iter() {
                    self.writer
                        .write_all(self.byte_char_panel[b as usize].as_bytes())?;
                    if idx == 8 * self.panels - 1 {
                        self.writer.write_all(
                            self.border_style
                                .outer_sep()
                                .encode_utf8(&mut [0; 4])
                                .as_bytes(),
                        )?;
                        self.writer.write_all(b"\n")?;
                    } else if idx % 8 == 7 {
                        self.writer.write_all(
                            self.border_style
                                .inner_sep()
                                .encode_utf8(&mut [0; 4])
                                .as_bytes(),
                        )?;
                    }
                    idx += 1;
                }

                // there is space left over at the end
                if idx < 8 * self.panels - 1 {
                    for i in idx..8 * self.panels {
                        self.writer.write_all(b" ")?;
                        if i == 8 * self.panels - 1 {
                            self.writer.write_all(
                                self.border_style
                                    .outer_sep()
                                    .encode_utf8(&mut [0; 4])
                                    .as_bytes(),
                            )?;
                            self.writer.write_all(b"\n")?;
                        } else if i % 8 == 7 {
                            self.writer.write_all(
                                self.border_style
                                    .inner_sep()
                                    .encode_utf8(&mut [0; 4])
                                    .as_bytes(),
                            )?;
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    pub fn print_byte(&mut self, b: u8) -> io::Result<()> {
        match self.squeezer.action() {
            SqueezeAction::Print => {
                self.writer.write_all(b"   ")?;
            }
            SqueezeAction::Ignore => {
                // print the byte
                self.writer
                    .write_all(self.byte_hex_panel[b as usize].as_bytes())?;
            }
            _ => unreachable!(),
        }
        // byte is last in panel
        if self.idx % 8 == 7 {
            // byte is last in last panel
            if self.idx % (8 * self.panels as u64) == 8 * self.panels as u64 - 1 {
                self.writer.write_all(b" ")?;
                self.writer.write_all(
                    self.border_style
                        .outer_sep()
                        .encode_utf8(&mut [0; 4])
                        .as_bytes(),
                )?;
            } else {
                self.writer.write_all(b" ")?;
                self.writer.write_all(
                    self.border_style
                        .inner_sep()
                        .encode_utf8(&mut [0; 4])
                        .as_bytes(),
                )?;
            }
        }

        Ok(())
    }

    pub fn print_line(&mut self, b: u8) -> io::Result<()> {
        let mut is_flushed = false;
        let old_active = self.squeezer.active();
        self.squeezer.process(b, self.idx);

        // the header should be the first thing printed
        if self.idx == 0 {
            self.print_header()?;
        }

        // flush the rest of the line buffer before continuing to write
        if old_active && !self.squeezer.active() {
            self.writer.write_all(
                self.border_style
                    .outer_sep()
                    .encode_utf8(&mut [0; 4])
                    .as_bytes(),
            )?;
            let old_idx = self.idx;
            self.idx -= self.line_buf.len() as u64;
            self.print_position_panel()?;
            for b in self.line_buf.clone() {
                self.print_byte(b)?;
                self.idx += 1;
            }
            self.idx = old_idx;
            is_flushed = true;
        }

        self.line_buf.push(b);

        if !self.squeezer.active() || self.squeezer.action() == SqueezeAction::Print {
            // print the left border and position panel if there's a new line
            if self.idx % (8 * self.panels as u64) == 0 && !is_flushed {
                self.writer.write_all(
                    self.border_style
                        .outer_sep()
                        .encode_utf8(&mut [0; 4])
                        .as_bytes(),
                )?;
                if self.show_position_panel {
                    self.print_position_panel()?;
                }
            }

            self.print_byte(b)?;

            if self.idx % (8 * self.panels as u64) == 8 * self.panels as u64 - 1 {
                if self.show_char_panel {
                    self.print_char_panel()?;
                } else {
                    self.writer.write_all(b"\n")?;
                }
                self.line_buf.clear();
            }
        } else {
            self.writer.write_all(&self.line_buf)?;
        }

        self.idx += 1;
        if self.idx % (8 * self.panels as u64) == 0 {
            self.line_buf.clear();
            self.squeezer.advance();
        }

        Ok(())
    }

    /// Loop through the given `Reader`, printing until the `Reader` buffer
    /// is exhausted.
    pub fn print_all<Reader: Read>(&mut self, reader: Reader) -> io::Result<()> {
        let mut is_empty = true;

        let buf = BufReader::new(reader);

        for b in buf.bytes() {
            if is_empty {
                is_empty = false;
            }
            if self.print_line(b?).is_err() {
                break;
            }
        }

        // special ending

        if is_empty {
            self.print_header()?;
            if self.show_position_panel {
                write!(self.writer, "{0:9}", "│")?;
            }
            write!(
                self.writer,
                "{0:2}{1:24}{0}{0:>26}",
                "│", "No content to print"
            )
            .ok();
            if self.show_char_panel {
                write!(self.writer, "{0:>9}{0:>9}", "│")?;
            }
            writeln!(self.writer)?;
        } else if self.squeezer.active() {
            // input was squeezed at the end
            write!(self.writer, "{}", self.border_style.outer_sep())?;
            self.print_position_panel()?;

            // print empty bytes
            for i in 0..8 * self.panels {
                write!(self.writer, "   ")?;
                if i % 8 == 7 {
                    if i % (8 * self.panels) == 8 * self.panels - 1 {
                        write!(self.writer, " {}", self.border_style.outer_sep())?;
                    } else {
                        write!(self.writer, " {}", self.border_style.inner_sep())?;
                    }
                }
            }

            // print empty char panel
            for i in 0..self.panels {
                write!(self.writer, "        ")?;
                if i == self.panels - 1 {
                    writeln!(self.writer, "{}", self.border_style.outer_sep())?;
                } else {
                    write!(self.writer, "{}", self.border_style.inner_sep())?;
                }
            }
        } else {
            // finish unfinished last line
            if self.idx % (8 * self.panels as u64) != 0 {
                for i in self.idx % (8 * self.panels as u64)..8 * self.panels as u64 {
                    // print empty byte space
                    write!(self.writer, "   ")?;
                    if i == (8 * self.panels - 1) as u64 {
                        write!(self.writer, " {}", self.border_style.outer_sep())?;
                    } else if i % 8 == 7 {
                        write!(self.writer, " {}", self.border_style.inner_sep())?;
                    }
                }
                if self.show_char_panel {
                    self.print_char_panel()?;
                } else {
                    writeln!(self.writer)?;
                }
            }
        }

        self.print_footer()?;

        self.writer.flush()?;

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
