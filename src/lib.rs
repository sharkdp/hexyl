pub(crate) mod input;

pub use input::*;

use std::io::{self, BufReader, Read, Write};

use owo_colors::{colors, Color};

pub enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

const COLOR_NULL: &[u8] = colors::BrightBlack::ANSI_FG.as_bytes();
const COLOR_OFFSET: &[u8] = colors::BrightBlack::ANSI_FG.as_bytes();
const COLOR_ASCII_PRINTABLE: &[u8] = colors::Cyan::ANSI_FG.as_bytes();
const COLOR_ASCII_WHITESPACE: &[u8] = colors::Green::ANSI_FG.as_bytes();
const COLOR_ASCII_OTHER: &[u8] = colors::Green::ANSI_FG.as_bytes();
const COLOR_NONASCII: &[u8] = colors::Yellow::ANSI_FG.as_bytes();
const COLOR_RESET: &[u8] = colors::Default::ANSI_FG.as_bytes();

pub enum ByteCategory {
    Null,
    AsciiPrintable,
    AsciiWhitespace,
    AsciiOther,
    NonAscii,
}

pub enum Endianness {
    Little,
    Big,
}

#[derive(PartialEq)]
enum Squeezer {
    Print,
    Delete,
    Ignore,
    Disabled,
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

    fn color(self) -> &'static [u8] {
        use crate::ByteCategory::*;

        match self.category() {
            Null => COLOR_NULL,
            AsciiPrintable => COLOR_ASCII_PRINTABLE,
            AsciiWhitespace => COLOR_ASCII_WHITESPACE,
            AsciiOther => COLOR_ASCII_OTHER,
            NonAscii => COLOR_NONASCII,
        }
    }

    fn as_char(self) -> char {
        use crate::ByteCategory::*;

        match self.category() {
            Null => '⋄',
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
    panels: u64,
    group_size: u8,
    base: Base,
    endianness: Endianness,
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
            group_size: 1,
            base: Base::Hexadecimal,
            endianness: Endianness::Big,
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

    pub fn num_panels(mut self, num: u64) -> Self {
        self.panels = num;
        self
    }

    pub fn group_size(mut self, num: u8) -> Self {
        self.group_size = num;
        self
    }

    pub fn with_base(mut self, base: Base) -> Self {
        self.base = base;
        self
    }

    pub fn endianness(mut self, endianness: Endianness) -> Self {
        self.endianness = endianness;
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
            self.group_size,
            self.base,
            self.endianness,
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
    show_color: bool,
    curr_color: Option<&'static [u8]>,
    border_style: BorderStyle,
    byte_hex_panel: Vec<String>,
    byte_char_panel: Vec<String>,
    // same as previous but in Fixed(242) gray color, for position panel
    byte_hex_panel_g: Vec<String>,
    squeezer: Squeezer,
    display_offset: u64,
    /// The number of panels to draw.
    panels: u64,
    squeeze_byte: usize,
    /// The number of octets per group.
    group_size: u8,
    /// The number of digits used to write the base.
    base_digits: u8,
    /// Whether to show groups in little or big endian ordering.
    endianness: Endianness,
}

impl<'a, Writer: Write> Printer<'a, Writer> {
    fn new(
        writer: &'a mut Writer,
        show_color: bool,
        show_char_panel: bool,
        show_position_panel: bool,
        border_style: BorderStyle,
        use_squeeze: bool,
        panels: u64,
        group_size: u8,
        base: Base,
        endianness: Endianness,
    ) -> Printer<'a, Writer> {
        Printer {
            idx: 0,
            line_buf: vec![0x0; 8 * panels as usize],
            writer,
            show_char_panel,
            show_position_panel,
            show_color,
            curr_color: None,
            border_style,
            byte_hex_panel: (0u8..=u8::MAX)
                .map(|i| match base {
                    Base::Binary => format!("{i:08b}"),
                    Base::Octal => format!("{i:03o}"),
                    Base::Decimal => format!("{i:03}"),
                    Base::Hexadecimal => format!("{i:02x}"),
                })
                .collect(),
            byte_char_panel: (0u8..=u8::MAX)
                .map(|i| format!("{}", Byte(i).as_char()))
                .collect(),
            byte_hex_panel_g: (0u8..=u8::MAX).map(|i| format!("{i:02x}")).collect(),
            squeezer: if use_squeeze {
                Squeezer::Ignore
            } else {
                Squeezer::Disabled
            },
            display_offset: 0,
            panels,
            squeeze_byte: 0x00,
            group_size,
            base_digits: match base {
                Base::Binary => 8,
                Base::Octal => 3,
                Base::Decimal => 3,
                Base::Hexadecimal => 2,
            },
            endianness,
        }
    }

    pub fn display_offset(&mut self, display_offset: u64) -> &mut Self {
        self.display_offset = display_offset;
        self
    }

    fn panel_sz(&self) -> usize {
        // add one to include the trailing space of a group
        let group_sz = self.base_digits as usize * self.group_size as usize + 1;
        let group_per_panel = 8 / self.group_size as usize;
        // add one to include the leading space
        1 + group_sz * group_per_panel
    }

    fn write_border(&mut self, border_elements: BorderElements) -> io::Result<()> {
        let h = border_elements.horizontal_line;
        let c = border_elements.column_separator;
        let l = border_elements.left_corner;
        let r = border_elements.right_corner;
        let h8 = h.to_string().repeat(8);
        let h_repeat = h.to_string().repeat(self.panel_sz());

        if self.show_position_panel {
            write!(self.writer, "{l}{h8}{c}")?;
        } else {
            write!(self.writer, "{l}")?;
        }

        for _ in 0..self.panels - 1 {
            write!(self.writer, "{h_repeat}{c}")?;
        }
        if self.show_char_panel {
            write!(self.writer, "{h_repeat}{c}")?;
        } else {
            write!(self.writer, "{h_repeat}")?;
        }

        if self.show_char_panel {
            for _ in 0..self.panels - 1 {
                write!(self.writer, "{h8}{c}")?;
            }
            writeln!(self.writer, "{h8}{r}")?;
        } else {
            writeln!(self.writer, "{r}")?;
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
        self.writer.write_all(
            self.border_style
                .outer_sep()
                .encode_utf8(&mut [0; 4])
                .as_bytes(),
        )?;
        if self.show_color {
            self.writer.write_all(COLOR_OFFSET)?;
        }
        if self.show_position_panel {
            match self.squeezer {
                Squeezer::Print => {
                    self.writer
                        .write_all(self.byte_char_panel[b'*' as usize].as_bytes())?;
                    if self.show_color {
                        self.writer.write_all(COLOR_RESET)?;
                    }
                    self.writer.write_all(b"       ")?;
                }
                Squeezer::Ignore | Squeezer::Disabled | Squeezer::Delete => {
                    let byte_index: [u8; 8] = (self.idx + self.display_offset).to_be_bytes();
                    let mut i = 0;
                    while byte_index[i] == 0x0 && i < 4 {
                        i += 1;
                    }
                    for &byte in byte_index.iter().skip(i) {
                        self.writer
                            .write_all(self.byte_hex_panel_g[byte as usize].as_bytes())?;
                    }
                    if self.show_color {
                        self.writer.write_all(COLOR_RESET)?;
                    }
                }
            }
            self.writer.write_all(
                self.border_style
                    .outer_sep()
                    .encode_utf8(&mut [0; 4])
                    .as_bytes(),
            )?;
        }
        Ok(())
    }

    fn print_char(&mut self, i: u64) -> io::Result<()> {
        match self.squeezer {
            Squeezer::Print | Squeezer::Delete => self.writer.write_all(b" ")?,
            Squeezer::Ignore | Squeezer::Disabled => {
                if let Some(&b) = self.line_buf.get(i as usize) {
                    if self.show_color && self.curr_color != Some(Byte(b).color()) {
                        self.writer.write_all(Byte(b).color())?;
                        self.curr_color = Some(Byte(b).color());
                    }
                    self.writer
                        .write_all(self.byte_char_panel[b as usize].as_bytes())?;
                } else {
                    self.squeezer = Squeezer::Print;
                }
            }
        }
        if i == 8 * self.panels - 1 {
            if self.show_color {
                self.writer.write_all(COLOR_RESET)?;
                self.curr_color = None;
            }
            self.writer.write_all(
                self.border_style
                    .outer_sep()
                    .encode_utf8(&mut [0; 4])
                    .as_bytes(),
            )?;
        } else if i % 8 == 7 {
            if self.show_color {
                self.writer.write_all(COLOR_RESET)?;
                self.curr_color = None;
            }
            self.writer.write_all(
                self.border_style
                    .inner_sep()
                    .encode_utf8(&mut [0; 4])
                    .as_bytes(),
            )?;
        }

        Ok(())
    }

    pub fn print_char_panel(&mut self) -> io::Result<()> {
        for i in 0..self.line_buf.len() {
            self.print_char(i as u64)?;
        }
        Ok(())
    }

    fn print_byte(&mut self, i: usize, b: u8) -> io::Result<()> {
        match self.squeezer {
            Squeezer::Print => {
                if !self.show_position_panel && i == 0 {
                    if self.show_color {
                        self.writer.write_all(COLOR_OFFSET)?;
                    }
                    self.writer
                        .write_all(self.byte_char_panel[b'*' as usize].as_bytes())?;
                    if self.show_color {
                        self.writer.write_all(COLOR_RESET)?;
                    }
                } else if i % (self.group_size as usize) == 0 {
                    self.writer.write_all(b" ")?;
                }
                for _ in 0..self.base_digits {
                    self.writer.write_all(b" ")?;
                }
            }
            Squeezer::Delete => self.writer.write_all(b"   ")?,
            Squeezer::Ignore | Squeezer::Disabled => {
                if i % (self.group_size as usize) == 0 {
                    self.writer.write_all(b" ")?;
                }
                if self.show_color && self.curr_color != Some(Byte(b).color()) {
                    self.writer.write_all(Byte(b).color())?;
                    self.curr_color = Some(Byte(b).color());
                }
                self.writer
                    .write_all(self.byte_hex_panel[b as usize].as_bytes())?;
            }
        }
        // byte is last in panel
        if i % 8 == 7 {
            if self.show_color {
                self.curr_color = None;
                self.writer.write_all(COLOR_RESET)?;
            }
            self.writer.write_all(b" ")?;
            // byte is last in last panel
            if i as u64 % (8 * self.panels) == 8 * self.panels - 1 {
                self.writer.write_all(
                    self.border_style
                        .outer_sep()
                        .encode_utf8(&mut [0; 4])
                        .as_bytes(),
                )?;
            } else {
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

    fn reorder_buf_to_little_endian(&self, buf: &mut Vec<u8>) {
        let n = buf.len();
        let group_sz = self.group_size as usize;

        for idx in (0..n).step_by(group_sz) {
            let remaining = n - idx;
            let total = remaining.min(group_sz);

            buf[idx..idx + total].reverse();
        }
    }

    pub fn print_bytes(&mut self) -> io::Result<()> {
        let mut buf = self.line_buf.clone();

        if matches!(self.endianness, Endianness::Little) {
            // reorder the buffer to the little endian format
            self.reorder_buf_to_little_endian(&mut buf);
        };

        for (i, &b) in buf.iter().enumerate() {
            self.print_byte(i, b)?;
        }
        Ok(())
    }

    /// Loop through the given `Reader`, printing until the `Reader` buffer
    /// is exhausted.
    pub fn print_all<Reader: Read>(&mut self, reader: Reader) -> io::Result<()> {
        let mut is_empty = true;

        let mut buf = BufReader::new(reader);

        let leftover = loop {
            // read a maximum of 8 * self.panels bytes from the reader
            if let Ok(n) = buf.read(&mut self.line_buf) {
                if n > 0 && n < 8 * self.panels as usize {
                    // if less are read, that indicates end of file after
                    if is_empty {
                        self.print_header()?;
                        is_empty = false;
                    }

                    // perform second check on read
                    if let Ok(0) = buf.read(&mut self.line_buf[n..]) {
                        self.line_buf.resize(n, 0);
                        break Some(n);
                    };
                } else if n == 0 {
                    // if no bytes are read, that indicates end of file
                    if self.squeezer == Squeezer::Delete {
                        // empty the last line when ending is squeezed
                        self.line_buf.clear();
                        break Some(0);
                    }
                    break None;
                }
            }
            if is_empty {
                self.print_header()?;
                is_empty = false;
            }

            // squeeze is active, check if the line is the same
            // skip print if still squeezed, otherwise print and deactivate squeeze
            if matches!(self.squeezer, Squeezer::Print | Squeezer::Delete) {
                if self
                    .line_buf
                    .chunks_exact(std::mem::size_of::<usize>())
                    .all(|w| usize::from_ne_bytes(w.try_into().unwrap()) == self.squeeze_byte)
                {
                    if self.squeezer == Squeezer::Delete {
                        self.idx += 8 * self.panels;
                        continue;
                    }
                } else {
                    self.squeezer = Squeezer::Ignore;
                }
            }

            // print the line
            self.print_position_panel()?;
            self.print_bytes()?;
            if self.show_char_panel {
                self.print_char_panel()?;
            }
            self.writer.write_all(b"\n")?;

            // increment index to next line
            self.idx += 8 * self.panels;

            // change from print to delete if squeeze is still active
            if self.squeezer == Squeezer::Print {
                self.squeezer = Squeezer::Delete;
            }

            // repeat the first byte in the line until it's a usize
            // compare that usize with each usize chunk in the line
            // if they are all the same, change squeezer to print
            let repeat_byte = (self.line_buf[0] as usize) * (usize::MAX / 255);
            if !matches!(self.squeezer, Squeezer::Disabled | Squeezer::Delete)
                && self
                    .line_buf
                    .chunks_exact(std::mem::size_of::<usize>())
                    .all(|w| usize::from_ne_bytes(w.try_into().unwrap()) == repeat_byte)
            {
                self.squeezer = Squeezer::Print;
                self.squeeze_byte = repeat_byte;
            };
        };

        // special ending

        if is_empty {
            self.base_digits = 2;
            self.print_header()?;
            if self.show_position_panel {
                write!(self.writer, "{0:9}", "│")?;
            }
            write!(
                self.writer,
                "{0:2}{1:2$}{0}{0:>3$}",
                "│",
                "No content",
                self.panel_sz() - 1,
                self.panel_sz() + 1,
            )?;
            if self.show_char_panel {
                write!(self.writer, "{0:>9}{0:>9}", "│")?;
            }
            writeln!(self.writer)?;
        } else if let Some(n) = leftover {
            // last line is incomplete
            self.print_position_panel()?;
            self.squeezer = Squeezer::Ignore;
            self.print_bytes()?;
            self.squeezer = Squeezer::Print;
            for i in n..8 * self.panels as usize {
                self.print_byte(i, 0)?;
            }
            if self.show_char_panel {
                self.squeezer = Squeezer::Ignore;
                self.print_char_panel()?;
                self.squeezer = Squeezer::Print;
                for i in n..8 * self.panels as usize {
                    self.print_char(i as u64)?;
                }
            }
            self.writer.write_all(b"\n")?;
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
            1,
            Base::Hexadecimal,
            Endianness::Big,
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
│        │ No content              │                         │        │        │
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
            1,
            Base::Hexadecimal,
            Endianness::Big,
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
            1,
            Base::Hexadecimal,
            Endianness::Big,
        );

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }

    #[test]
    fn squeeze_works() {
        let input = io::Cursor::new(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│*       │                         ┊                         │        ┊        │
│00000020│ 00                      ┊                         │⋄       ┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();
        assert_print_all_output(input, expected_string);
    }

    #[test]
    fn squeeze_nonzero() {
        let input = io::Cursor::new(b"000000000000000000000000000000000");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 30 30 30 30 30 30 30 30 ┊ 30 30 30 30 30 30 30 30 │00000000┊00000000│
│*       │                         ┊                         │        ┊        │
│00000020│ 30                      ┊                         │0       ┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();
        assert_print_all_output(input, expected_string);
    }

    #[test]
    fn squeeze_multiple_panels() {
        let input = io::Cursor::new(b"0000000000000000000000000000000000000000000000000");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬─────────────────────────┬────────┬────────┬────────┐
│00000000│ 30 30 30 30 30 30 30 30 ┊ 30 30 30 30 30 30 30 30 ┊ 30 30 30 30 30 30 30 30 │00000000┊00000000┊00000000│
│*       │                         ┊                         ┊                         │        ┊        ┊        │
│00000030│ 30                      ┊                         ┊                         │0       ┊        ┊        │
└────────┴─────────────────────────┴─────────────────────────┴─────────────────────────┴────────┴────────┴────────┘
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
            3,
            1,
            Base::Hexadecimal,
            Endianness::Big,
        );

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }
}
