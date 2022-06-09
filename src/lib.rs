pub(crate) mod input;
pub mod squeezer;

pub use input::*;

use std::io::{self, Read, Write};

use ansi_term::Color::Fixed;
use ansi_term::{Color, Style};
use once_cell::sync::Lazy;

use crate::squeezer::{SqueezeAction, Squeezer};

const BUFFER_SIZE: usize = 256;

static STYLE_NULL_16: Lazy<Style> = Lazy::new(|| Color::Black.bold()); // grey
static STYLE_OFFSET_16: Lazy<Style> = Lazy::new(|| Color::Black.bold()); // grey

static STYLE_NULL_8BIT: Lazy<Style> = Lazy::new(|| Fixed(242).normal()); // grey
static STYLE_OFFSET_8BIT: Lazy<Style> = Lazy::new(|| Fixed(242).normal()); // grey

static STYLE_ASCII_PRINTABLE: Lazy<Style> = Lazy::new(|| Color::Cyan.normal());
static STYLE_ASCII_WHITESPACE: Lazy<Style> = Lazy::new(|| Color::Green.normal());
static STYLE_ASCII_OTHER: Lazy<Style> = Lazy::new(|| Color::Purple.normal());
static STYLE_NONASCII: Lazy<Style> = Lazy::new(|| Color::Yellow.normal());

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

    fn style(self, use_8_bit_color: bool) -> &'static Style {
        use crate::ByteCategory::*;

        match self.category() {
            Null => {
                if use_8_bit_color {
                    &STYLE_NULL_8BIT
                } else {
                    &STYLE_NULL_16
                }
            }
            AsciiPrintable => &STYLE_ASCII_PRINTABLE,
            AsciiWhitespace => &STYLE_ASCII_WHITESPACE,
            AsciiOther => &STYLE_ASCII_OTHER,
            NonAscii => &STYLE_NONASCII,
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

pub struct Printer<'a, Writer: Write> {
    idx: u64,
    /// The raw bytes used as input for the current line.
    raw_line: Vec<u8>,
    /// The buffered line built with each byte, ready to print to writer.
    buffer_line: Vec<u8>,
    writer: &'a mut Writer,
    show_color: bool,
    use_8_bit_color: bool,
    show_char_panel: bool,
    show_position_panel: bool,
    border_style: BorderStyle,
    header_was_printed: bool,
    byte_hex_panel: Vec<String>,
    byte_char_panel: Vec<String>,
    squeezer: Squeezer,
    display_offset: u64,
}

impl<'a, Writer: Write> Printer<'a, Writer> {
    pub fn new(
        writer: &'a mut Writer,
        show_color: bool,
        use_8_bit_color: bool,
        show_char_panel: bool,
        show_position_panel: bool,
        border_style: BorderStyle,
        use_squeeze: bool,
    ) -> Printer<'a, Writer> {
        Printer {
            idx: 1,
            raw_line: vec![],
            buffer_line: vec![],
            writer,
            show_color,
            use_8_bit_color,
            show_char_panel,
            show_position_panel,
            border_style,
            header_was_printed: false,
            byte_hex_panel: (0u8..=u8::max_value())
                .map(|i| {
                    let byte_hex = format!("{:02x} ", i);
                    if show_color {
                        Byte(i).style(use_8_bit_color).paint(byte_hex).to_string()
                    } else {
                        byte_hex
                    }
                })
                .collect(),
            byte_char_panel: show_char_panel
                .then(|| {
                    (0u8..=u8::max_value())
                        .map(|i| {
                            let byte_char = format!("{}", Byte(i).as_char());
                            if show_color {
                                Byte(i).style(use_8_bit_color).paint(byte_char).to_string()
                            } else {
                                byte_char
                            }
                        })
                        .collect()
                })
                .unwrap_or_default(),
            squeezer: Squeezer::new(use_squeeze),
            display_offset: 0,
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

        write!(self.writer, "{h25}{c}{h25}", c = c, h25 = h25).ok();

        if self.show_char_panel {
            writeln!(self.writer, "{c}{h8}{c}{h8}{r}", c = c, h8 = h8, r = r).ok();
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

    fn get_offset_style(&self) -> Style {
        if self.use_8_bit_color {
            *STYLE_OFFSET_8BIT
        } else {
            *STYLE_OFFSET_16
        }
    }

    fn print_position_panel(&mut self) {
        if !self.show_position_panel {
            write!(&mut self.buffer_line, "{} ", self.border_style.outer_sep()).ok();
            return;
        }

        let style = self.get_offset_style();
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

            if idx == 8 {
                let _ = write!(&mut self.buffer_line, "{}", self.border_style.inner_sep());
            }

            idx += 1;
        }

        if len < 8 {
            let _ = writeln!(
                &mut self.buffer_line,
                "{0:1$}{3}{0:2$}{4}",
                "",
                8 - len,
                8,
                self.border_style.inner_sep(),
                self.border_style.outer_sep(),
            );
        } else {
            let _ = writeln!(
                &mut self.buffer_line,
                "{0:1$}{2}",
                "",
                16 - len,
                self.border_style.outer_sep()
            );
        }
    }

    pub fn print_byte(&mut self, b: u8) -> io::Result<()> {
        if self.idx % 16 == 1 {
            self.print_header();
            self.print_position_panel();
        }

        write!(&mut self.buffer_line, "{}", self.byte_hex_panel[b as usize])?;
        self.raw_line.push(b);

        self.squeezer.process(b, self.idx);

        match self.idx % 16 {
            8 => {
                let _ = write!(&mut self.buffer_line, "{} ", self.border_style.inner_sep());
            }
            0 => {
                self.print_textline()?;
            }
            _ => {}
        }

        self.idx += 1;

        Ok(())
    }

    pub fn print_textline(&mut self) -> io::Result<()> {
        let len = self.raw_line.len();

        if len == 0 {
            if self.squeezer.active() {
                self.print_position_panel();
                let _ = writeln!(
                    &mut self.buffer_line,
                    "{0:1$}{4}{0:2$}{5}{0:3$}{4}{0:3$}{5}",
                    "",
                    24,
                    25,
                    8,
                    self.border_style.inner_sep(),
                    self.border_style.outer_sep(),
                );
                self.writer.write_all(&self.buffer_line)?;
            }
            return Ok(());
        }

        let squeeze_action = self.squeezer.action();

        if squeeze_action != SqueezeAction::Delete {
            if len < 8 {
                let _ = write!(
                    &mut self.buffer_line,
                    "{0:1$}{3}{0:2$}{4}",
                    "",
                    3 * (8 - len),
                    1 + 3 * 8,
                    self.border_style.inner_sep(),
                    self.border_style.outer_sep(),
                );
            } else {
                let _ = write!(
                    &mut self.buffer_line,
                    "{0:1$}{2}",
                    "",
                    3 * (16 - len),
                    self.border_style.outer_sep()
                );
            }
        }

        self.print_char_panel();

        match squeeze_action {
            SqueezeAction::Print => {
                self.buffer_line.clear();
                let style = self.get_offset_style();
                let asterisk = if self.show_color {
                    format!("{}", style.paint("*"))
                } else {
                    String::from("*")
                };
                let _ = writeln!(
                    &mut self.buffer_line,
                    "{5}{0}{1:2$}{5}{1:3$}{6}{1:3$}{5}{1:4$}{6}{1:4$}{5}",
                    asterisk,
                    "",
                    7,
                    25,
                    8,
                    self.border_style.outer_sep(),
                    self.border_style.inner_sep(),
                );
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
            true,
            BorderStyle::Unicode,
            true,
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
            true,
            BorderStyle::Unicode,
            true,
        );
        printer.display_offset(0xdeadbeef);

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }
}
