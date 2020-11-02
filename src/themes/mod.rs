use ansi_term::Style;
use core::ops::Deref;

pub(super) const EMPTY_STYLE: Style = ansi_term::Style {
    foreground:       None,
    background:       None,
    is_bold:          false,
    is_dimmed:        false,
    is_italic:        false,
    is_underline:     false,
    is_blink:         false,
    is_reverse:       false,
    is_hidden:        false,
    is_strikethrough: false,
};

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
    inner: [Style; 11],
}

impl Deref for CategoryColors {
    type Target = [Style; 11];
    fn deref(&self) -> &Self::Target {&self.inner}
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
    /// Style of magic numbers of a binary input format (e.g. ELF: 7f 45 4c 46).
    pub magic_number: Style,
    /// Style of padding-bytes.
    pub padding:      Style,
    /// Style of an integer value.
    pub integer:      Style,
    /// Style of a floating point value.
    pub float:        Style,
    /// Style of a pointer or offset value.
    pub pointer:      Style,
    /// Style of a length field.
    pub length:       Style,
}

/// Just for readability.
macro_rules! fakeMap {
    ($Key:expr, $Value:expr) => {$Value}
}

impl CategoryTheme {
    pub(crate) fn to_colors(&self) -> CategoryColors {
      CategoryColors {
          inner: [
              fakeMap!(ByteCategory::Null,        self.null        ),
              fakeMap!(ByteCategory::Printable,   self.printable   ),
              fakeMap!(ByteCategory::Whitespace,  self.whitespace  ),
              fakeMap!(ByteCategory::Control,     self.control     ),
              fakeMap!(ByteCategory::Invalid,     self.invalid     ),
              fakeMap!(ByteCategory::MagicNumber, self.magic_number),
              fakeMap!(ByteCategory::Padding,     self.padding     ),
              fakeMap!(ByteCategory::Integer,     self.integer     ),
              fakeMap!(ByteCategory::Float,       self.float       ),
              fakeMap!(ByteCategory::Pointer,     self.pointer     ),
              fakeMap!(ByteCategory::Length,      self.length      ),
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

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use super::{CategoryColors, CategoryTheme, EMPTY_STYLE};

    #[test]
    fn to_colors() {
        let colors_left = CategoryColors {
            inner: [
                style!(Color::Blue),
                style!(Color::Red),
                style!(Color::Green),
                style!(Color::Yellow),
                style!(Color::Cyan),
                style!(Color::White),
                style!(Color::Black),
                style!(Color::Purple),
                style!(Color::Fixed(242)),
                ansi_term::Style {
                    foreground:       None,
                    background:       Some(Color::Green),
                    is_bold:          true,
                    is_dimmed:        false,
                    is_italic:        true,
                    is_underline:     false,
                    is_blink:         false,
                    is_reverse:       false,
                    is_hidden:        false,
                    is_strikethrough: true,
                },
                EMPTY_STYLE,
            ],
        };

        let colors_right = CategoryTheme {
            null:         style!(Color::Blue      ),
            printable:    style!(Color::Red       ),
            whitespace:   style!(Color::Green     ),
            control:      style!(Color::Yellow    ),
            invalid:      style!(Color::Cyan      ),
            magic_number: style!(Color::White     ),
            padding:      style!(Color::Black     ),
            integer:      style!(Color::Purple    ),
            float:        style!(Color::Fixed(242)),
            pointer:      ansi_term::Style {
                foreground:       None,
                background:       Some(Color::Green),
                is_bold:          true,
                is_dimmed:        false,
                is_italic:        true,
                is_underline:     false,
                is_blink:         false,
                is_reverse:       false,
                is_hidden:        false,
                is_strikethrough: true,
            },
            length:       EMPTY_STYLE,
        }.to_colors();

        for index in 0..11 {
            assert_eq!(colors_left[index], colors_right[index]);
        }
    }
}
