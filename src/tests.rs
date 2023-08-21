use super::*;

#[test]
fn unit_multipliers() {
    use Unit::*;
    assert_eq!(Kilobyte.get_multiplier(), 1000 * Byte.get_multiplier());
    assert_eq!(Megabyte.get_multiplier(), 1000 * Kilobyte.get_multiplier());
    assert_eq!(Gigabyte.get_multiplier(), 1000 * Megabyte.get_multiplier());
    assert_eq!(Terabyte.get_multiplier(), 1000 * Gigabyte.get_multiplier());

    assert_eq!(Kibibyte.get_multiplier(), 1024 * Byte.get_multiplier());
    assert_eq!(Mebibyte.get_multiplier(), 1024 * Kibibyte.get_multiplier());
    assert_eq!(Gibibyte.get_multiplier(), 1024 * Mebibyte.get_multiplier());
    assert_eq!(Tebibyte.get_multiplier(), 1024 * Gibibyte.get_multiplier());
}

#[test]
fn test_process_sign() {
    use ByteOffsetKind::*;
    use ByteOffsetParseError::*;
    assert_eq!(process_sign_of("123"), Ok(("123", ForwardFromBeginning)));
    assert_eq!(process_sign_of("+123"), Ok(("123", ForwardFromLastOffset)));
    assert_eq!(process_sign_of("-123"), Ok(("123", BackwardFromEnd)));
    assert_eq!(process_sign_of("-"), Err(EmptyAfterSign));
    assert_eq!(process_sign_of("+"), Err(EmptyAfterSign));
    assert_eq!(process_sign_of(""), Err(Empty));
}

#[test]
fn test_parse_as_hex() {
    assert_eq!(try_parse_as_hex_number("73"), None);
    assert_eq!(try_parse_as_hex_number("0x1337"), Some(Ok(0x1337)));
    assert!(matches!(try_parse_as_hex_number("0xnope"), Some(Err(_))));
    assert!(matches!(try_parse_as_hex_number("0x-1"), Some(Err(_))));
}

#[test]
fn extract_num_and_unit() {
    use ByteOffsetParseError::*;
    use Unit::*;
    // byte is default unit
    assert_eq!(extract_num_and_unit_from("4"), Ok((4, Byte)));
    // blocks are returned without customization
    assert_eq!(
        extract_num_and_unit_from("2blocks"),
        Ok((2, Block { custom_size: None }))
    );
    // no normalization is performed
    assert_eq!(extract_num_and_unit_from("1024kb"), Ok((1024, Kilobyte)));

    // unit without number results in error
    assert_eq!(
        extract_num_and_unit_from("gib"),
        Err(EmptyWithUnit("gib".to_string()))
    );
    // empty string results in error
    assert_eq!(extract_num_and_unit_from(""), Err(Empty));
    // an invalid unit results in an error
    assert_eq!(
        extract_num_and_unit_from("25litres"),
        Err(InvalidUnit("litres".to_string()))
    );
}

#[test]
fn test_parse_byte_offset() {
    use ByteOffsetParseError::*;

    macro_rules! success {
        ($input: expr, $expected_kind: ident $expected_value: expr) => {
            success!($input, $expected_kind $expected_value; block_size: DEFAULT_BLOCK_SIZE)
        };
        ($input: expr, $expected_kind: ident $expected_value: expr; block_size: $block_size: expr) => {
            assert_eq!(
                parse_byte_offset($input, PositiveI64::new($block_size).unwrap()),
                Ok(
                    ByteOffset {
                        value: NonNegativeI64::new($expected_value).unwrap(),
                        kind: ByteOffsetKind::$expected_kind,
                    }
                ),
            );
        };
    }

    macro_rules! error {
        ($input: expr, $expected_err: expr) => {
            assert_eq!(
                parse_byte_offset($input, PositiveI64::new(DEFAULT_BLOCK_SIZE).unwrap()),
                Err($expected_err),
            );
        };
    }

    success!("0", ForwardFromBeginning 0);
    success!("1", ForwardFromBeginning 1);
    success!("1", ForwardFromBeginning 1);
    success!("100", ForwardFromBeginning 100);
    success!("+100", ForwardFromLastOffset 100);

    success!("0x0", ForwardFromBeginning 0);
    success!("0xf", ForwardFromBeginning 15);
    success!("0xdeadbeef", ForwardFromBeginning 3_735_928_559);

    success!("1KB", ForwardFromBeginning 1000);
    success!("2MB", ForwardFromBeginning 2000000);
    success!("3GB", ForwardFromBeginning 3000000000);
    success!("4TB", ForwardFromBeginning 4000000000000);
    success!("+4TB", ForwardFromLastOffset 4000000000000);

    success!("1GiB", ForwardFromBeginning 1073741824);
    success!("2TiB", ForwardFromBeginning 2199023255552);
    success!("+2TiB", ForwardFromLastOffset 2199023255552);

    success!("0xff", ForwardFromBeginning 255);
    success!("0xEE", ForwardFromBeginning 238);
    success!("+0xFF", ForwardFromLastOffset 255);

    success!("1block", ForwardFromBeginning 512; block_size: 512);
    success!("2block", ForwardFromBeginning 1024; block_size: 512);
    success!("1block", ForwardFromBeginning 4; block_size: 4);
    success!("2block", ForwardFromBeginning 8; block_size: 4);

    // empty string is invalid
    error!("", Empty);
    // These are also bad.
    error!("+", EmptyAfterSign);
    error!("-", EmptyAfterSign);
    error!("K", InvalidNumAndUnit("K".to_owned()));
    error!("k", InvalidNumAndUnit("k".to_owned()));
    error!("m", InvalidNumAndUnit("m".to_owned()));
    error!("block", EmptyWithUnit("block".to_owned()));
    // leading/trailing space is invalid
    error!(" 0", InvalidNumAndUnit(" 0".to_owned()));
    error!("0 ", InvalidUnit(" ".to_owned()));
    // Signs after the hex prefix make no sense
    error!("0x-12", SignFoundAfterHexPrefix('-'));
    // This was previously accepted but shouldn't be.
    error!("0x+12", SignFoundAfterHexPrefix('+'));
    // invalid suffix
    error!("1234asdf", InvalidUnit("asdf".to_owned()));
    // bad numbers
    error!("asdf1234", InvalidNumAndUnit("asdf1234".to_owned()));
    error!("a1s2d3f4", InvalidNumAndUnit("a1s2d3f4".to_owned()));
    // multiplication overflows u64
    error!("20000000TiB", UnitMultiplicationOverflow);

    assert!(
        match parse_byte_offset("99999999999999999999", PositiveI64::new(512).unwrap()) {
            // We can't check against the kind of the `ParseIntError`, so we'll just make sure it's the
            // same as trying to do the parse directly.
            Err(ParseNum(e)) => e == "99999999999999999999".parse::<i64>().unwrap_err(),
            _ => false,
        }
    );
}
