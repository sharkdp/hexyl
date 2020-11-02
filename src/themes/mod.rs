use ansi_term::Style;
use core::ops::Deref;

/// Create a style just with the given foreground color.
macro_rules! style {
    ($Color:expr) => {
        ansi_term::Style {
            foreground:       Some($Color),
            background:       None,
            is_bold:          false,
            is_dimmed:        false,
            is_italic:        false,
            is_underline:     false,
            is_blink:         false,
            is_reverse:       false,
            is_hidden:        false,
            is_strikethrough: false,
        }
    };
}

/// The Hexylamine color scheme.
pub mod hexylamine;
pub use hexylamine::Hexylamine;

/// Look-up-table for `paint_char` and `paint_byte` of module `formats`.
/// This allows the somewhat faster formatting.
pub(crate) struct CategoryColors {
    inner: [Style; 5],
}

impl Deref for CategoryColors {
    type Target = [Style; 5];
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Style of bytes in different `ByteCategory`s.
pub struct CategoryTheme {
    /// Style of the \0-byte.
    pub null:         Style,
    /// Style of printable characters (e.g. »A«).
    pub printable:    Style,
    /// Style of whitespace characters (e.g. \t).
    pub whitespace:   Style,
    /// Style of any other control-character (e.g. \a).
    pub control:      Style,
    /// Style of invalid characters of the current encoding.
    pub invalid:      Style,
}

/// Just for readability.
macro_rules! fakeMap {
    ($Key:expr, $Value:expr) => {$Value}
}

impl CategoryTheme {
    pub(crate) fn to_colors(&self) -> CategoryColors {
      CategoryColors {
          inner: [
              fakeMap!(ByteCategory::Null,            self.null      ),
              fakeMap!(ByteCategory::AsciiPrintable,  self.printable ),
              fakeMap!(ByteCategory::AsciiWhitespace, self.whitespace),
              fakeMap!(ByteCategory::AsciiOther,      self.control   ),
              fakeMap!(ByteCategory::NonAscii,        self.invalid   ),
          ]
      }
    }
}

/// A Theme.
/// ToDo: Serde
pub struct Theme {
    /// Style of the offset value in the hexdump-table.
    pub offset:   Style,
    /// Style of the border of the hexdump-table.
    pub border:   Style,
    /// Style of the characters of each category.
    pub category: CategoryTheme,
}
