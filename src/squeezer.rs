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
const LSIZE: u64 = 16;

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

    pub fn process(&mut self, b: u8, i: u64) {
        use self::SqueezeState::*;
        if self.state == Disabled {
            return;
        }
        let eq = b == self.byte;

        if i % LSIZE == 0 {
            if !eq {
                self.state = Probe;
            } else {
                self.state = match self.state {
                    NoSqueeze => Probe,
                    Probe => SqueezeActiveFirstLine,
                    SqueezeActiveFirstLine => SqueezeFirstLine,
                    SqueezeFirstLine => SqueezeActive,
                    SqueezeActive => Squeeze,
                    Squeeze => SqueezeActive,
                    Disabled => Disabled,
                };
            }
        } else if !eq {
            if i % LSIZE == 1 {
                self.state = Probe;
            } else if i % LSIZE != 1 {
                self.state = NoSqueeze;
            }
        }

        self.byte = b;
    }

    pub fn active(&self) -> bool {
        use self::SqueezeState::*;
        matches!(
            self.state,
            Squeeze | SqueezeActive | SqueezeFirstLine | SqueezeActiveFirstLine
        )
    }

    pub fn action(&self) -> SqueezeAction {
        match self.state {
            SqueezeState::SqueezeFirstLine => SqueezeAction::Print,
            SqueezeState::Squeeze => SqueezeAction::Delete,
            _ => SqueezeAction::Ignore,
        }
    }

    pub fn advance(&mut self) {
        match self.state {
            SqueezeState::SqueezeFirstLine => {
                self.state = SqueezeState::SqueezeActive;
            }
            SqueezeState::Squeeze => {
                self.state = SqueezeState::SqueezeActive;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LSIZE_USIZE: usize = LSIZE as usize;

    #[test]
    fn three_same_lines() {
        const LINES: usize = 3;
        let v = vec![0u8; LINES * LSIZE_USIZE];
        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Print,  // print squeeze symbol
            SqueezeAction::Delete, // delete reoccurring line
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
        }
    }

    #[test]
    fn incomplete_while_squeeze() {
        // fourth line only has 12 bytes and should be printed
        let v = vec![0u8; 3 * LSIZE_USIZE + 12];
        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Print,  // print squeeze symbol
            SqueezeAction::Delete, // delete reoccurring line
            SqueezeAction::Ignore, // last line only 12 bytes, print it
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            assert_eq!(s.action(), exp[line]);
            s.advance();
        }
    }

    #[test]
    /// all three lines are different, print all
    fn three_different_lines() {
        let mut v: Vec<u8> = vec![];
        v.extend(vec![0u8; 16]);
        v.extend(vec![1u8; 16]);
        v.extend(vec![2u8; 16]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Ignore, // different
            SqueezeAction::Ignore, // different
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            assert_eq!(action, exp[line]);
            s.advance();
        }
    }

    #[test]
    /// first two lines same, hence squeeze symbol, third line diff, hence
    /// print
    fn one_squeeze_no_delete() {
        const LINES: usize = 3;
        let mut v = vec![0u8; (LINES - 1) * LSIZE_USIZE];
        v.extend(vec![1u8; 16]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Print,  // print squeeze symbol
            SqueezeAction::Ignore, // different lines, print again
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
        }
    }

    #[test]
    /// First line all eq, 2nd half eq with first line, then change
    fn second_line_different() {
        const LINES: usize = 2;
        let mut v = vec![0u8; (LINES - 1) * LSIZE_USIZE];
        v.extend(vec![0u8; 8]);
        v.extend(vec![1u8; 8]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Ignore, // print squeeze symbol
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
        }
    }

    #[test]
    /// all three lines never become squeeze candidate (diff within line)
    fn never_squeeze_candidate() {
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
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore, // first line, print as is
            SqueezeAction::Ignore, // print squeeze symbol
            SqueezeAction::Ignore, // print squeeze symbol
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
        }
    }

    #[test]
    fn mix_everything() {
        let mut v = vec![];
        v.extend(vec![10u8; 16]); // print
        v.extend(vec![20u8; 16]); // print
        v.extend(vec![0u8; 16]); // print
        v.extend(vec![0u8; 16]); // *
        v.extend(vec![10u8; 16]); // print
        v.extend(vec![20u8; 16]); // print
        v.extend(vec![0u8; 16]); // print
        v.extend(vec![0u8; 16]); // *
        v.extend(vec![0u8; 16]); // delete
        v.extend(vec![0u8; 16]); // delete*
        v.extend(vec![20u8; 16]); // print
        v.extend(vec![0u8; 12]); // print, only 12 bytes

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore,
            SqueezeAction::Ignore,
            SqueezeAction::Ignore,
            SqueezeAction::Print,
            SqueezeAction::Ignore,
            SqueezeAction::Ignore,
            SqueezeAction::Ignore,
            SqueezeAction::Print,
            SqueezeAction::Delete,
            SqueezeAction::Delete,
            SqueezeAction::Ignore,
            SqueezeAction::Ignore,
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
        }
    }

    #[test]
    fn last_char_diff() {
        // see issue #62
        let mut v = vec![];
        v.extend(vec![20u8; 16]);
        v.extend(vec![20u8; 15]);
        v.push(61);
        v.extend(vec![20u8; 16]);
        v.extend(vec![20u8; 16]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore, // print as is
            SqueezeAction::Ignore, // print as is
            SqueezeAction::Ignore, // print as is
            SqueezeAction::Print,  // print '*' char
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            assert_eq!(s.action(), exp[line]);
            s.advance();
        }
    }

    #[test]
    fn first_char_diff() {
        // see issue #62
        let mut v = vec![];
        v.extend(vec![20u8; 16]);
        v.push(61);
        v.extend(vec![20u8; 15]);
        v.extend(vec![20u8; 16]);

        let mut s = Squeezer::new(true);
        // just initialized
        assert_eq!(s.action(), SqueezeAction::Ignore);
        s.advance();

        let exp = vec![
            SqueezeAction::Ignore, // print as is
            SqueezeAction::Ignore, // print as is
            SqueezeAction::Ignore, // print as is
        ];

        let mut idx = 1;
        for (line, z) in v.chunks(LSIZE_USIZE).enumerate() {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            assert_eq!(s.action(), exp[line]);
            s.advance();
        }
    }
}
