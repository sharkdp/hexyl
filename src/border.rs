pub(crate) struct BorderElements {
    pub(crate) left_corner:      char,
    pub(crate) horizontal_line:  char,
    pub(crate) column_separator: char,
    pub(crate) right_corner:     char,
}

/// Style of the Border arround bytes and characters.
pub enum BorderStyle {
    /// Use special unicode characters for border.
    /// This is the nicest one.
    Unicode,
    /// Use simple ascii characters for border.
    /// Looks okish.
    Ascii,
    /// No nation, no border.
    None,
}

impl BorderStyle {
    pub(crate) fn header_elems(&self) -> Option<BorderElements> {
        match self {
            BorderStyle::Unicode => Some(BorderElements {
                left_corner:      '┌',
                horizontal_line:  '─',
                column_separator: '┬',
                right_corner:     '┐',
            }),
            BorderStyle::Ascii => Some(BorderElements {
                left_corner:      '+',
                horizontal_line:  '-',
                column_separator: '+',
                right_corner:     '+',
            }),
            BorderStyle::None => None,
        }
    }

    pub(crate) fn footer_elems(&self) -> Option<BorderElements> {
        match self {
            BorderStyle::Unicode => Some(BorderElements {
                left_corner:      '└',
                horizontal_line:  '─',
                column_separator: '┴',
                right_corner:     '┘',
            }),
            BorderStyle::Ascii => Some(BorderElements {
                left_corner:      '+',
                horizontal_line:  '-',
                column_separator: '+',
                right_corner:     '+',
            }),
            BorderStyle::None => None,
        }
    }

    pub(crate) fn outer_sep(&self) -> char {
        match self {
            BorderStyle::Unicode => '│',
            BorderStyle::Ascii   => '|',
            BorderStyle::None    => ' ',
        }
    }

    pub(crate) fn inner_sep(&self) -> char {
        match self {
            BorderStyle::Unicode => '┊',
            BorderStyle::Ascii   => '|',
            BorderStyle::None    => ' ',
        }
    }
}
