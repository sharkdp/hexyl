use hexyl::squeezer::{Squeezer, LSIZE, SqueezeAction};

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

    let mut line = 0;
    let mut idx = 1;
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        let action = s.action();
        assert_eq!(action, exp[line]);
        line += 1;
    }
}

#[test]
fn incomplete_while_squeeze() {
    // fourth line only has 12 bytes and should be printed
    let v = vec![0u8; 3 * LSIZE + 12];
    let mut s = Squeezer::new(true);
    // just initialized
    assert_eq!(s.action(), SqueezeAction::Ignore);

    let exp = vec![
        SqueezeAction::Ignore, // first line, print as is
        SqueezeAction::Print,  // print squeeze symbol
        SqueezeAction::Delete, // delete reoccurring line
        SqueezeAction::Ignore, // last line only 12 bytes, print it
    ];

    let mut line = 0;
    let mut idx = 1;
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        assert_eq!(s.action(), exp[line]);
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

    let exp = vec![
        SqueezeAction::Ignore, // first line, print as is
        SqueezeAction::Ignore, // different
        SqueezeAction::Ignore, // different
    ];

    let mut line = 0;
    let mut idx = 1;
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        let action = s.action();
        assert_eq!(action, exp[line]);
        line += 1;
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

    let mut line = 0;
    let mut idx = 1;
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        let action = s.action();
        assert_eq!(action, exp[line]);
        line += 1;
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

    let mut line = 0;
    let mut idx = 1;
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        let action = s.action();
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

    let exp = vec![
        SqueezeAction::Ignore, // first line, print as is
        SqueezeAction::Ignore, // print squeeze symbol
        SqueezeAction::Ignore, // print squeeze symbol
    ];

    let mut line = 0;
    let mut idx = 1;
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        let action = s.action();
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
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        let action = s.action();
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

    let exp = vec![
        SqueezeAction::Ignore, // print as is
        SqueezeAction::Ignore, // print as is
        SqueezeAction::Ignore, // print as is
        SqueezeAction::Print,  // print '*' char
    ];

    let mut line = 0;
    let mut idx = 1;
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        assert_eq!(s.action(), exp[line]);
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

    let exp = vec![
        SqueezeAction::Ignore, // print as is
        SqueezeAction::Ignore, // print as is
        SqueezeAction::Ignore, // print as is
    ];

    let mut line = 0;
    let mut idx = 1;
    for z in v.chunks(LSIZE) {
        for i in z {
            s.process(*i, idx);
            idx += 1;
        }
        assert_eq!(s.action(), exp[line]);
        line += 1;
    }}
