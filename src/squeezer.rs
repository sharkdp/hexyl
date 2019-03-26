#[derive(Debug, PartialEq)]
enum SqueezeState {
    /// not enabled
    Disabled,
    /// Will be set from all states if equal condition can't be hold up.
    /// Set if previous byte is not equal the current processed byte.
    NoSqueeze,
    /// Valid for a whole line to identify if it is candidate for squeezing
    Probe,
    /// Squeeze line parsing is active, but EOL is not reached yet
    SqueezeActive,
    /// Squeeze line, EOL is reached, will influence the action
    Squeeze,
    /// same as Squeeze, however this is only for the first line after
    /// the squeeze candidate has been set.
    SqueezeFirstLine,
    /// same as SqueezeActive, however this is only for the first line after
    /// the squeeze candidate has been set.
    SqueezeActiveFirstLine,
}

pub struct Squeezer {
    state: SqueezeState,
    byte: u8,
}

#[derive(Debug, PartialEq)]
pub enum SqueezeAction {
    Ignore,
    Print,
    Delete,
}

/// line size
const LSIZE: usize = 16;

impl Squeezer {
    pub fn new(enabled: bool) -> Squeezer {
        Squeezer {
            state: if enabled {
                SqueezeState::Probe
            } else {
                SqueezeState::Disabled
            },
            byte: 0,
        }
    }

    pub fn process(&mut self, b: &u8, i: &usize) {
        use self::SqueezeState::*;
        if self.state == Disabled {
            return;
        }
        let eq = *b == self.byte;

        if i % LSIZE == 0 {
            self.state = match self.state {
                NoSqueeze => Probe,
                Probe => SqueezeActiveFirstLine,
                SqueezeActiveFirstLine => SqueezeFirstLine,
                SqueezeFirstLine => SqueezeActive,
                SqueezeActive => Squeeze,
                Squeeze => SqueezeActive,
                Disabled => Disabled,
            };
        } else if !eq {
            if (i % LSIZE == 1 && self.state != Probe) || i % LSIZE != 1 {
                self.state = NoSqueeze;
            }
        }

        self.byte = b.clone();
    }

    pub fn active(&self) -> bool {
        use self::SqueezeState::*;
        match self.state {
            Squeeze | SqueezeActive | SqueezeFirstLine | SqueezeActiveFirstLine => true,
            _ => false,
        }
    }

    pub fn action(&mut self) -> SqueezeAction {
        match self.state {
            SqueezeState::SqueezeFirstLine => {
                self.state = SqueezeState::SqueezeActive;
                SqueezeAction::Print
            }
            SqueezeState::Squeeze => {
                self.state = SqueezeState::SqueezeActive;
                SqueezeAction::Delete
            }
            _ => SqueezeAction::Ignore,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_same_lines() {
        const LINES: usize = 3;
        let v = vec![0u8; LINES * LSIZE];
        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Print,  // print squeeze symbol
            SqueezeAction::Delete, // delete reoccurring line
        ];

        for z in 0..LINES {
            for i in z * LSIZE..(z + 1) * LSIZE {
                s.process(&v[i], &(i + 1));
            }
            assert_eq!(s.action(), exp[z]);
        }
    }

    #[test]
    fn incomplete_while_squeeze() {
        const LINES: usize = 3;
        // third line only has 12 bytes and should be printed
        let v = vec![0u8; LINES * LSIZE + 12];
        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Print,  // print squeeze symbol
            SqueezeAction::Delete, // delete reoccurring line
            SqueezeAction::Ignore, // last line only 12 bytes, print it
        ];

        for z in 0..LINES {
            for i in z * LSIZE..(z + 1) * LSIZE {
                s.process(&v[i], &(i + 1));
            }
            assert_eq!(s.action(), exp[z]);
        }

        for i in LSIZE * LINES..LINES * LSIZE + 12 {
            s.process(&v[i], &(i + 1));
        }
        assert_eq!(s.action(), exp[3]);
    }

    #[test]
    /// all three lines are different, print all
    fn three_different_lines() {
        const LINES: usize = 3;
        let mut v: Vec<u8> = vec![];
        v.extend(vec![0u8; 16]);
        v.extend(vec![1u8; 16]);
        v.extend(vec![2u8; 16]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Ignore, // different
            SqueezeAction::Ignore, // different
        ];

        for z in 0..LINES {
            for i in z * LSIZE..(z + 1) * LSIZE {
                s.process(&v[i], &(i + 1));
            }
            assert_eq!(s.action(), exp[z]);
        }
    }

    #[test]
    /// first two lines same, hence squeeze symbol, third line diff, hence
    /// print
    fn one_squeeze_no_delete() {
        const LINES: usize = 3;
        let mut v = vec![0u8; (LINES - 1) * LSIZE];
        v.extend(vec![1u8; 16]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Print,  // print squeeze symbol
            SqueezeAction::Ignore, // different lines, print again
        ];

        for z in 0..LINES {
            for i in z * LSIZE..(z + 1) * LSIZE {
                s.process(&v[i], &(i + 1));
            }
            assert_eq!(s.action(), exp[z]);
        }
    }

    #[test]
    /// First line all eq, 2nd half eq with first line, then change
    fn second_line_different() {
        const LINES: usize = 2;
        let mut v = vec![0u8; (LINES - 1) * LSIZE];
        v.extend(vec![0u8; 8]);
        v.extend(vec![1u8; 8]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Ignore, // print squeeze symbol
        ];

        for z in 0..LINES {
            for i in z * LSIZE..(z + 1) * LSIZE {
                s.process(&v[i], &(i + 1));
            }
            assert_eq!(s.action(), exp[z]);
        }
    }

    #[test]
    /// all three lines never become squeeze candidate (diff within line)
    fn never_squeeze_candidate() {
        const LINES: usize = 3;
        let mut v = vec![];
        v.extend(vec![0u8; 8]);
        v.extend(vec![1u8; 8]);
        v.extend(vec![0u8; 8]);
        v.extend(vec![1u8; 8]);
        v.extend(vec![0u8; 8]);
        v.extend(vec![1u8; 8]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Ignore, // print squeeze symbol
            SqueezeAction::Ignore, // print squeeze symbol
        ];

        for z in 0..LINES {
            for i in z * LSIZE..(z + 1) * LSIZE {
                s.process(&v[i], &(i + 1));
            }
            assert_eq!(s.action(), exp[z]);
        }
    }
}
