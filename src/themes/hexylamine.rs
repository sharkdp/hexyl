use ansi_term::Color;
use super::{CategoryTheme, EMPTY_STYLE, Theme};

/// The Hexylamine-Theme.
/// This is the default-colorscheme.
#[allow(non_upper_case_globals)]
pub const Hexylamine: Theme = Theme {
    offset: style!(Color::Fixed(242)),
    border: EMPTY_STYLE,
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
    },
};
