/// American Standard Code for Information Interchange.
mod ascii;

use std::borrow::Cow;
use ascii::{AsciiFormatter};
use crate::themes::CategoryColors;

/// One formatted byte.
pub(crate) struct Byte {
    pub(crate) byte:       u8,
    pub(crate) category:   ByteCategory,
    pub(crate) character:  &'static str,
}

impl Byte {
    pub(crate) fn paint_byte (
        &self,
        colors:   &Option<CategoryColors>,
        hextable: [&'static str; 256],
    ) -> Cow<'static, str> {
        if let Some(colors) = colors {
            Cow::Owned (
                colors[self.category as usize]
                .paint(hextable[self.byte as usize])
                .to_string()
            )
        } else {
            Cow::Borrowed (
                hextable[self.byte as usize]
            )
        }
    }

    pub(crate) fn paint_char(&self, colors: &Option<CategoryColors>) -> Cow<'static, str> {
      if let Some(colors) = colors {
            Cow::Owned (
                colors[self.category as usize]
                .paint(self.character)
                .to_string()
            )
        } else {
          Cow::Borrowed (
              self.character
          )
        }
    }
}

/// The Category of the byte.
/// This basically tells the printer how to color the byte.
#[derive(Clone,Copy)]
pub enum ByteCategory {
    /// The \0-byte.
    Null,
    /// A printable character (e.g. »A«).
    Printable,
    /// A whitespace character (e.g. \t).
    Whitespace,
    /// Any other control-character (e.g. \a).
    Control,
    /// Invalid characters of the current encoding.
    Invalid,
    /// Magic number of a binary input format (e.g. ELF: 7f 45 4c 46).
    MagicNumber,
    /// Bytes for padding.
    Padding,
    /// Integer value.
    Integer,
    /// Floating point value.
    Float,
    /// Pointer or offset value.
    Pointer,
    /// Length field.
    Length,
}

pub(crate) trait ByteFormatter {
    /// Give buffer to the formatter to parse and return an iterator over its bytes.
    fn parse(&mut self, buffer: &[u8]) -> Vec<Byte>;
}

/// Input protocol- or file-format.
pub enum InputFormat {
    /// ASCII-encoded text.
    Ascii,
}

impl InputFormat {
    pub(crate) fn get(self) -> Box<dyn ByteFormatter> {
        match self {
          InputFormat::Ascii  => Box::new(AsciiFormatter),
        }
    }
}