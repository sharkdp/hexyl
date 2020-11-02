/// Some nice borders around the dump.
pub mod border;
/// Style protocol/file-format to highlight the structure-fields.
pub mod formats;
pub(crate) mod input;
/// The look-up-table of the hexadecimal-value of every byte.
mod lookup;
pub mod squeezer;
/// Customable themes.
pub mod themes;

pub use input::*;

use std::io::{self, Read, Write};

use crate::formats::{Byte, ByteFormatter};
use crate::lookup::{LookUpTable, LOOKUP_HEX_LOWER};
use crate::squeezer::{SqueezeAction, Squeezer};
use crate::themes::CategoryColors;

// Reexports
pub use crate::border::BorderStyle;
pub use crate::formats::InputFormat;
pub use crate::themes::Theme;

const BUFFER_SIZE: usize = 256;

struct PrinterStyle {
    border_prefix:    String,
    border_style:     BorderStyle,
    border_suffix:    String,
    offset_prefix:    String,
    offset_suffix:    String,
    category_colors:  Option<CategoryColors>,
}

impl PrinterStyle {
    fn new(theme: Option<Theme>, border_style: BorderStyle) -> Self {
        let (
            border_prefix,
            border_suffix,
            offset_prefix,
            offset_suffix,
            category_colors,
        ) = if let Some(theme) = theme {
            (
                theme.border.prefix().to_string(),
                theme.border.suffix().to_string(),
                theme.offset.prefix().to_string(),
                theme.offset.suffix().to_string(),
                Some(theme.category.to_colors()),
            )
        } else {
            (
              String::new(),
              String::new(),
              String::new(),
              String::new(),
              None,
            )
        };
        Self {
            border_prefix,
            border_style,
            border_suffix,
            offset_prefix,
            offset_suffix,
            category_colors,
        }
    }
}

pub struct Printer<'a, Writer: Write> {
    index: u64,
    /// The raw bytes used as input for the current line.
    raw_line: Vec<Byte>,
    /// The buffered line built with each byte, ready to print to writer.
    buffer_line: Vec<u8>,
    writer: &'a mut Writer,
    /// The style to use for nice output.
    style: PrinterStyle,
    header_was_printed: bool,
    squeezer: Squeezer,
    /// Look-up-table of the hexadecimal-value of every byte.
    hex_table: LookUpTable,
    display_offset: u64,
    /// The formatter in use for formatting the input-stream, e.g. as ASCII- or ELF-file.
    formatter: Box<dyn ByteFormatter>,
}

impl<'a, Writer: Write> Printer<'a, Writer> {
    pub fn new(
        writer: &'a mut Writer,
        theme: Option<Theme>,
        border_style: BorderStyle,
        input_format: InputFormat,
        use_squeeze: bool,
    ) -> Printer<'a, Writer> {
        Printer {
            index: 1,
            raw_line: vec![],
            buffer_line: vec![],
            writer,
            style: PrinterStyle::new(theme, border_style),
            header_was_printed: false,
            squeezer: Squeezer::new(use_squeeze),
            hex_table: LOOKUP_HEX_LOWER,
            display_offset: 0,
            formatter: input_format.get(),
        }
    }

    pub fn display_offset(&mut self, display_offset: u64) -> &mut Self {
        self.display_offset = display_offset;
        self
    }

    fn header(&mut self) {
        if let Some(border_elements) = self.style.border_style.header_elems() {
            let h = border_elements.horizontal_line;
            let h8 = h.to_string().repeat(8);
            let h25 = h.to_string().repeat(25);

            writeln!(
                self.writer,
                "{bp}{lc}{h8}{sep}{h25}{sep}{h25}{sep}{h8}{sep}{h8}{rc}{bs}",
                lc  = border_elements.left_corner,
                sep = border_elements.column_separator,
                rc  = border_elements.right_corner,
                h8  = h8,
                h25 = h25,
                bp  = self.style.border_prefix,
                bs  = self.style.border_suffix,
            )
            .ok();
        }
    }

    fn footer(&mut self) {
        if let Some(border_elements) = self.style.border_style.footer_elems() {
            let h   = border_elements.horizontal_line;
            writeln!(
                self.writer,
                "{bp}{lc}{h8}{sep}{h25}{sep}{h25}{sep}{h8}{sep}{h8}{rc}{bs}",
                lc  = border_elements.left_corner,
                sep  = border_elements.column_separator,
                rc  = border_elements.right_corner,
                h8  = h.to_string().repeat(8),
                h25 = h.to_string().repeat(25),
                bp  = self.style.border_prefix,
                bs  = self.style.border_suffix,
            )
            .ok();
        }
    }

    fn print_position_indicator(&mut self) -> io::Result<()> {
        if !self.header_was_printed {
            self.header();
            self.header_was_printed = true;
        }

        write!(
            &mut self.buffer_line,
            "{bp}{sep}{bs}{op}{off:08x}{os}{bp}{sep}{bs} ",
            off = self.index - 1 + self.display_offset,
            sep = self.style.border_style.outer_sep(),
            bp  = self.style.border_prefix,
            bs  = self.style.border_suffix,
            op  = self.style.offset_prefix,
            os  = self.style.offset_suffix,
        )
    }

    fn print_byte(&mut self, byte: Byte) -> io::Result<()> {
        if self.index % 16 == 1 {
            self.print_position_indicator()?;
        }

        let raw_byte = byte.byte;
        write! (
            &mut self.buffer_line,
            "{} ",
            byte.paint_byte (
                &self.style.category_colors,
                self.hex_table,
            )
        )?;
        self.raw_line.push(byte);

        self.squeezer.process(raw_byte, self.index);

        match self.index % 16 {
            8 => write! (
                    &mut self.buffer_line,
                    "{bp}{sep}{bs} ",
                    sep = self.style.border_style.inner_sep(),
                    bp  = self.style.border_prefix,
                    bs  = self.style.border_suffix,
                )?,
            0 =>  self.print_textline()?,
            _ => {}
        }

        self.index += 1;

        Ok(())
    }

    fn print_textline(&mut self) -> io::Result<()> {
        let length = self.raw_line.len();

        if length == 0 {
            if self.squeezer.active() {
                self.print_position_indicator()?;
                writeln!(
                    &mut self.buffer_line,
                    "{bp}{w:h24$}{is}{w:h25$}{os}{w:h8$}{is}{w:h8$}{os}{bs}",
                    w   = "",
                    h24 = 24,
                    h25 = 25,
                    h8  = 8,
                    is  = self.style.border_style.inner_sep(),
                    os  = self.style.border_style.outer_sep(),
                    bp  = self.style.border_prefix,
                    bs  = self.style.border_suffix,
                )?;
                self.writer.write_all(&self.buffer_line)?;
            }
            return Ok(());
        }

        let squeeze_action = self.squeezer.action();

        if squeeze_action != SqueezeAction::Delete {
            if length < 8 {
                write!(
                    &mut self.buffer_line,
                    "{bp}{w:hl$}{is}{w:h25$}{os}{bs}",
                    w   = "",
                    hl  = 3 * (8 - length),
                    h25 = 1 + 3 * 8,
                    is  = self.style.border_style.inner_sep(),
                    os  = self.style.border_style.outer_sep(),
                    bp  = self.style.border_prefix,
                    bs  = self.style.border_suffix,
                )?;
            } else {
                write!(
                    &mut self.buffer_line,
                    "{bp}{w:hl$}{sep}{bs}",
                    w   = "",
                    hl  = 3 * (16 - length),
                    sep = self.style.border_style.outer_sep(),
                    bp  = self.style.border_prefix,
                    bs  = self.style.border_suffix,
                )?;
            }

            for (index,byte) in self.raw_line.iter().enumerate() {
                write!(
                    &mut self.buffer_line,
                    "{}",
                    byte.paint_char(&self.style.category_colors),
                )?;

                if index == 7 {
                    write! (
                      &mut self.buffer_line,
                      "{bp}{sep}{bs}",
                      sep = self.style.border_style.inner_sep(),
                      bp  = self.style.border_prefix,
                      bs  = self.style.border_suffix,
                  )?;
                }
            }

            if length < 8 {
                writeln!(
                    &mut self.buffer_line,
                    "{p}{w:hl$}{i}{w:h8$}{o}{s}",
                    w  = "",
                    hl = 8 - length,
                    h8 = 8,
                    i  = self.style.border_style.inner_sep(),
                    o  = self.style.border_style.outer_sep(),
                    p  = self.style.border_prefix,
                    s  = self.style.border_suffix,
                )?;
            } else {
                writeln!(
                    &mut self.buffer_line,
                    "{p}{w:h$}{o}{s}",
                    w  = "",
                    h  = 16 - length,
                    o  = self.style.border_style.outer_sep(),
                    p  = self.style.border_prefix,
                    s  = self.style.border_suffix,
                )?;
            }
        }

        match squeeze_action {
            SqueezeAction::Print => {
                self.buffer_line.clear();
                writeln!(
                    &mut self.buffer_line,
                    "{bp}{o}{bs}{op}*{os}{bp}{w:h7$}{o}{w:h25$}{i}{w:h25$}{o}{w:h8$}{i}{w:h8$}{o}{bs}",
                    w   = "",
                    h7  = 7,
                    h25 = 25,
                    h8  = 8,
                    o   = self.style.border_style.outer_sep(),
                    i   = self.style.border_style.inner_sep(),
                    bp  = self.style.border_prefix,
                    bs  = self.style.border_suffix,
                    op  = self.style.offset_prefix,
                    os  = self.style.offset_suffix,
                )?;
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

            for byte in self.formatter.parse(&buffer[..size]) {
                let res = self.print_byte(byte);

                if res.is_err() {
                    // Broken pipe
                    break 'mainloop;
                }
            }
        }

        // Finish last line
        self.print_textline().ok();

        if !self.header_was_printed {
            self.header();
            writeln! (
                self.writer,
                "{p}│        │ No content to print     │                         │        │        │{s}",
                p = self.style.border_prefix,
                s = self.style.border_suffix,
            ).ok();
        }
        self.footer();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::str;

    use super::*;

    fn assert_print_all_output<Reader: Read>(input: Reader, expected_string: String) -> () {
        let mut output = vec![];
        let mut printer = Printer::new(&mut output, None, BorderStyle::Unicode, InputFormat::Ascii, true);

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
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
        let mut printer: Printer<Vec<u8>> =
            Printer::new(&mut output, None, BorderStyle::Unicode, InputFormat::Ascii, true);
        printer.display_offset(0xdeadbeef);

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }
}
