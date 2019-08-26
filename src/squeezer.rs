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
pub const LSIZE: usize = 16;

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

    pub fn process(&mut self, b: u8, i: usize) {
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

