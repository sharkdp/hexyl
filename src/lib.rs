pub(crate) mod colors;
pub(crate) mod input;

pub use colors::*;
pub use input::*;

use std::io::{self, BufReader, Read, Write};

use clap::ValueEnum;

pub enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

#[derive(Copy, Clone)]
pub enum ByteCategory {
    Null,
    AsciiPrintable,
    AsciiWhitespace,
    AsciiOther,
    NonAscii,
}

pub enum IncludeMode {
    File(String), // filename
    Stdin,
    Slice,
    Off,
}

#[derive(Copy, Clone, Debug, Default, ValueEnum)]
#[non_exhaustive]
pub enum CharacterTable {
    /// Show printable ASCII characters as-is, '⋄' for NULL bytes, ' ' for
    /// space, '_' for other ASCII whitespace, '•' for other ASCII characters,
    /// and '×' for non-ASCII bytes.
    #[default]
    Default,

    /// Show printable ASCII as-is, ' ' for space, '.' for everything else.
    Ascii,

    /// Show printable EBCDIC as-is, ' ' for space, '.' for everything else.
    #[value(name = "codepage-1047")]
    CP1047,

    /// Uses code page 437 (for non-ASCII bytes).
    #[value(name = "codepage-437")]
    CP437,
}

#[derive(Copy, Clone, Debug, Default, ValueEnum)]
pub enum Endianness {
    /// Print out groups in little-endian format.
    Little,

    /// Print out groups in big-endian format.
    #[default]
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

    fn as_char(self, character_table: CharacterTable) -> char {
        use crate::ByteCategory::*;
        match character_table {
            CharacterTable::Default => match self.category() {
                Null => '⋄',
                AsciiPrintable => self.0 as char,
                AsciiWhitespace if self.0 == 0x20 => ' ',
                AsciiWhitespace => '_',
                AsciiOther => '•',
                NonAscii => '×',
            },
            CharacterTable::Ascii => match self.category() {
                Null => '.',
                AsciiPrintable => self.0 as char,
                AsciiWhitespace if self.0 == 0x20 => ' ',
                AsciiWhitespace => '.',
                AsciiOther => '.',
                NonAscii => '.',
            },
            CharacterTable::CP1047 => CP1047[self.0 as usize],
            CharacterTable::CP437 => CP437[self.0 as usize],
        }
    }
}

struct BorderElements {
    left_corner: char,
    horizontal_line: char,
    column_separator: char,
    right_corner: char,
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum BorderStyle {
    /// Draw a border with Unicode characters.
    #[default]
    Unicode,

    /// Draw a border with ASCII characters.
    Ascii,

    /// Do not draw a border at all.
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
    character_table: CharacterTable,
    include_mode: IncludeMode,
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
            character_table: CharacterTable::Default,
            include_mode: IncludeMode::Off,
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

    pub fn character_table(mut self, character_table: CharacterTable) -> Self {
        self.character_table = character_table;
        self
    }

    pub fn include_mode(mut self, include: IncludeMode) -> Self {
        self.include_mode = include;
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
            self.character_table,
            self.include_mode,
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
    /// Whether to show groups in little or big endian format.
    endianness: Endianness,
    /// Whether to output in C include file style.
    include_mode: IncludeMode,
}

impl<'a, Writer: Write> Printer<'a, Writer> {
    #![allow(clippy::too_many_arguments)]
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
        character_table: CharacterTable,
        include_mode: IncludeMode,
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
                .map(|i| format!("{}", Byte(i).as_char(character_table)))
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
            include_mode,
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
                    self.writer.write_all(b"*")?;
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

    fn reorder_buffer_to_little_endian(&self, buf: &mut [u8]) {
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
            self.reorder_buffer_to_little_endian(&mut buf);
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

        // special handler for include mode
        match &self.include_mode {
            // Input from a file
            // Output like `unsigned char <filename>[] = { ... }; unsigned int <filename>_len = ...;`
            IncludeMode::File(filename) => {
                let mut data = Vec::new();
                buf.read_to_end(&mut data)?;
                // convert non-alphanumeric characters to '_'
                let var_name = filename
                    .chars()
                    .map(|c| if c.is_alphanumeric() { c } else { '_' })
                    .collect::<String>();
                writeln!(self.writer, "unsigned char {}[] = {{", var_name)?;

                for (i, byte) in data.iter().enumerate() {
                    if i % 12 == 0 {
                        if i > 0 {
                            writeln!(self.writer, ",")?;
                        }
                        // first line
                        write!(self.writer, "  ")?;
                    } else {
                        write!(self.writer, ", ")?;
                    }
                    write!(self.writer, "0x{:02x}", byte)?;
                }
                writeln!(self.writer, "\n}};")?;
                writeln!(
                    self.writer,
                    "unsigned int {}_len = {};",
                    var_name,
                    data.len()
                )?;
                return Ok(());
            }
            IncludeMode::Stdin | IncludeMode::Slice => {
                let mut data = Vec::new();
                buf.read_to_end(&mut data)?;
                for (i, byte) in data.iter().enumerate() {
                    if i % 12 == 0 {
                        if i > 0 {
                            if i < data.len() {
                                write!(self.writer, ",")?;
                            }
                            writeln!(self.writer)?;
                        }
                        // first line
                        write!(self.writer, "  ")?;
                    } else {
                        write!(self.writer, ", ")?;
                    }
                    write!(self.writer, "0x{:02x}", byte)?;
                }
                writeln!(self.writer)?;
                return Ok(());
            }
            IncludeMode::Off => {}
        }

        let leftover = loop {
            // read a maximum of 8 * self.panels bytes from the reader
            if let Ok(n) = buf.read(&mut self.line_buf) {
                if n > 0 && n < 8 * self.panels as usize {
                    // if less are read, that indicates end of file after
                    if is_empty {
                        self.print_header()?;
                        is_empty = false;
                    }
                    let mut leftover = n;
                    // loop until input is ceased
                    if let Some(s) = loop {
                        if let Ok(n) = buf.read(&mut self.line_buf[leftover..]) {
                            leftover += n;
                            // there is no more input being read
                            if n == 0 {
                                self.line_buf.resize(leftover, 0);
                                break Some(leftover);
                            }
                            // amount read has exceeded line buffer
                            if leftover >= 8 * self.panels as usize {
                                break None;
                            }
                        }
                    } {
                        break Some(s);
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

            if is_empty {
                self.writer.flush()?;
                is_empty = false;
            }

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
            self.squeezer = Squeezer::Ignore;
            self.print_position_panel()?;
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
            CharacterTable::Default,
            IncludeMode::Off,
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
            CharacterTable::Default,
            IncludeMode::Off,
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
            CharacterTable::Default,
            IncludeMode::Off,
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
            CharacterTable::Default,
            IncludeMode::Off,
        );

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }

    // issue#238
    #[test]
    fn display_offset_in_last_line() {
        let input = io::Cursor::new(b"AAAAAAAAAAAAAAAACCCC");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 41 41 41 41 41 41 41 41 ┊ 41 41 41 41 41 41 41 41 │AAAAAAAA┊AAAAAAAA│
│00000010│ 43 43 43 43             ┊                         │CCCC    ┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();
        assert_print_all_output(input, expected_string);
    }
}
