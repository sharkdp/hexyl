#[derive(Debug, PartialEq)]
enum SqueezeState {
    /// Not enabled.
    Disabled,
    /// Will be set from all states if equal condition can't be hold up.
    /// Set if previous byte is not equal the current processed byte.
    NoSqueeze,
    /// Valid for a whole line to identify if it is candidate for squeezing.
    Probe,
    /// Squeeze line parsing is active, but EOL is not reached yet.
    SqueezeActive,
    /// Squeeze line, EOL is reached, will influence the action.
    Squeeze,
    /// Same as Squeeze, however this is only for the first line after
    ///   the squeeze candidate has been set.
    SqueezeFirstLine,
    /// Same as SqueezeActive, however this is only for the first line after
    ///   the squeeze candidate has been set.
    SqueezeActiveFirstLine,
}

/// Squeezer for `byte`.
pub(crate) struct Squeezer {
    state: SqueezeState,
    byte: u8,
}

#[derive(Debug, PartialEq)]
pub(crate) enum SqueezeAction {
    Ignore,
    Print,
    Delete,
}

/// Line size.
const LSIZE: u64 = 16;

impl Squeezer {
    /// Construct a new `Squeezer`.
    pub(crate) fn new(enabled: bool) -> Squeezer {
        Squeezer {
            state: if enabled {
                SqueezeState::Probe
            } else {
                SqueezeState::Disabled
            },
            byte: 0,
        }
    }

    pub(crate) fn action(&self) -> SqueezeAction {
        match self.state {
            SqueezeState::SqueezeFirstLine => SqueezeAction::Print,
            SqueezeState::Squeeze => SqueezeAction::Delete,
            _ => SqueezeAction::Ignore,
        }
    }

    /// Is squeezer active?
    pub(crate) fn active(&self) -> bool {
        use self::SqueezeState::*;
        match self.state {
            Squeeze | SqueezeActive | SqueezeFirstLine | SqueezeActiveFirstLine => true,
            _ => false,
        }
    }

    pub(crate) fn advance(&mut self) {
        match self.state {
            SqueezeState::SqueezeFirstLine | SqueezeState::Squeeze => {
                self.state = SqueezeState::SqueezeActive;
            },
            _ => {},
        }
    }

    /// Process a single byte.
    pub(crate) fn process(&mut self, b: u8, i: u64) -> bool {
        use self::SqueezeState::*;
        if self.state == Disabled {
            return false;
        }
        let eq = b == self.byte;

        if i % LSIZE == 0 {
            if !eq {
                self.state = Probe;
            } else {
                self.state = match self.state {
                    NoSqueeze               => Probe,
                    Probe                   => SqueezeActiveFirstLine,
                    SqueezeActiveFirstLine  => SqueezeFirstLine,
                    SqueezeFirstLine        => SqueezeActive,
                    SqueezeActive           => Squeeze,
                    Squeeze                 => SqueezeActive,
                    //  Untestable, because unrechable.
                    Disabled                => unreachable!(),
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
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LSIZE_USIZE: usize = LSIZE as usize;

    fn advance_helper(squeezer: &mut Squeezer, input: SqueezeState, output: SqueezeState) {
      squeezer.state = input;
      squeezer.advance();
      assert_eq!(squeezer.state, output);
    }

    #[test]
    fn advance() {
        let mut squeezer = Squeezer::new(true);
        advance_helper(&mut squeezer, SqueezeState::Disabled,               SqueezeState::Disabled              );
        advance_helper(&mut squeezer, SqueezeState::NoSqueeze,              SqueezeState::NoSqueeze             );
        advance_helper(&mut squeezer, SqueezeState::Probe,                  SqueezeState::Probe                 );
        advance_helper(&mut squeezer, SqueezeState::SqueezeActive,          SqueezeState::SqueezeActive         );
        advance_helper(&mut squeezer, SqueezeState::Squeeze,                SqueezeState::SqueezeActive         );
        advance_helper(&mut squeezer, SqueezeState::SqueezeFirstLine,       SqueezeState::SqueezeActive         );
        advance_helper(&mut squeezer, SqueezeState::SqueezeActiveFirstLine, SqueezeState::SqueezeActiveFirstLine);
    }

    fn process_helper(squeezer: &mut Squeezer, input: SqueezeState, output: SqueezeState) {
      squeezer.state = input;
      squeezer.process(0,0);
      assert_eq!(squeezer.state, output);
    }

    #[test]
    fn process() {
        let mut squeezer = Squeezer::new(false);
        assert_eq!(false, squeezer.process(0,0));
        let mut squeezer = Squeezer::new(true);
        assert_eq!(true,  squeezer.process(0,0));
        process_helper(&mut squeezer, SqueezeState::Disabled,               SqueezeState::Disabled              );
        process_helper(&mut squeezer, SqueezeState::NoSqueeze,              SqueezeState::Probe                 );
        process_helper(&mut squeezer, SqueezeState::Probe,                  SqueezeState::SqueezeActiveFirstLine);
        process_helper(&mut squeezer, SqueezeState::SqueezeActive,          SqueezeState::Squeeze               );
        process_helper(&mut squeezer, SqueezeState::Squeeze,                SqueezeState::SqueezeActive         );
        process_helper(&mut squeezer, SqueezeState::SqueezeFirstLine,       SqueezeState::SqueezeActive         );
        process_helper(&mut squeezer, SqueezeState::SqueezeActiveFirstLine, SqueezeState::SqueezeFirstLine      );
    }

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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
            line += 1;
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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            assert_eq!(s.action(), exp[line]);
            s.advance();
            line += 1;
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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            assert_eq!(action, exp[line]);
            s.advance();
            line += 1;
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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
            line += 1;
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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
            line += 1;
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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
            line += 1;
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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            let action = s.action();
            s.advance();
            assert_eq!(action, exp[line]);
            line += 1;
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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            assert_eq!(s.action(), exp[line]);
            s.advance();
            line += 1;
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

        let mut line = 0;
        let mut idx = 1;
        for z in v.chunks(LSIZE_USIZE) {
            for i in z {
                s.process(*i, idx);
                idx += 1;
            }
            assert_eq!(s.action(), exp[line]);
            s.advance();
            line += 1;
        }
    }
}
