use ansi_term::{Color, Style};
use super::{CategoryTheme, Theme};

/// The Hexylamine-Theme.
/// This is the default-colorscheme.
#[allow(non_upper_case_globals)]
pub const Hexylamine: Theme = Theme {
    offset: style!(Color::Fixed(242)),
    border: Style {
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
    },
    category: CategoryTheme {
      null:         style!(Color::Fixed(242)),
      printable:    style!(Color::Cyan      ),
      whitespace:   style!(Color::Green     ),
      control:      style!(Color::Purple    ),
      invalid:      style!(Color::Yellow    ),
      magic_number: style!(Color::Blue      ),
      padding:      style!(Color::Blue      ),
      integer:      style!(Color::Blue      ),
      float:        style!(Color::Blue      ),
      pointer:      style!(Color::Blue      ),
      length:       style!(Color::Blue      ),
    }
};
