use core::iter::Iterator;
use super::{Byte, ByteCategory, ByteFormatter};

macro_rules! c {()                    => {(ByteCategory::Control,     "•"       )};}
macro_rules! i {()                    => {(ByteCategory::Invalid,     "×"       )};}
macro_rules! n {()                    => {(ByteCategory::Null,        "0"       )};}
macro_rules! p {($Character:literal)  => {(ByteCategory::Printable,   $Character)};}
macro_rules! w {($Character:literal)  => {(ByteCategory::Whitespace,  $Character)};}

const LOOKUP_EBCDIC: [(ByteCategory, &str); 256] = [
    n!(),     c!(),     c!(),     c!(),     c!(),     w!("_" ), c!(),     c!(),
    c!(),     c!(),     c!(),     c!(),     w!("_" ), w!("_" ), c!(),     c!(),
    c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),
    c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),
    c!(),     c!(),     c!(),     c!(),     c!(),     w!("_" ), c!(),     c!(),
    c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),
    i!(),     i!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),
    c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     i!(),     c!(),
    w!(" " ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     p!("¢" ), p!("." ), p!("<" ), p!("(" ), p!("+" ), p!("|" ),
    p!("&" ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     p!("!" ), p!("$" ), p!("*" ), p!(")" ), p!(";" ), p!("¬" ),
    p!("-" ), p!("/" ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     p!("¦" ), p!("," ), p!("%" ), p!("_" ), p!(">" ), p!("?" ),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     p!("`" ), p!(":" ), p!("#" ), p!("@" ), p!("'" ), p!("=" ), p!("\""),
    i!(),     p!("a" ), p!("b" ), p!("c" ), p!("d" ), p!("e" ), p!("f" ), p!("g" ),
    p!("h" ), p!("i" ), i!(),     i!(),     i!(),     i!(),     i!(),     p!("±" ),
    i!(),     p!("j" ), p!("k" ), p!("l" ), p!("m" ), p!("n" ), p!("o" ), p!("p" ),
    p!("q" ), p!("r" ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     p!("~" ), p!("s" ), p!("t" ), p!("u" ), p!("v" ), p!("w" ), p!("x" ),
    p!("y" ), p!("z" ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    p!("^" ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     p!("[" ), p!("]" ), i!(),     i!(),     i!(),     i!(),
    p!("{" ), p!("A" ), p!("B" ), p!("C" ), p!("D" ), p!("E" ), p!("F" ), p!("G" ),
    p!("H" ), p!("I" ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    p!("}" ), p!("J" ), p!("K" ), p!("L" ), p!("M" ), p!("N" ), p!("O" ), p!("P" ),
    p!("Q" ), p!("R" ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    p!("\\"), i!(),     p!("S" ), p!("T" ), p!("U" ), p!("V" ), p!("W" ), p!("X" ),
    p!("Y" ), p!("Z" ), i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    p!("0" ), p!("1" ), p!("2" ), p!("3" ), p!("4" ), p!("5" ), p!("6" ), p!("7" ),
    p!("8" ), p!("9" ), i!(),     i!(),     i!(),     i!(),     i!(),     c!(),
];

/// The EBCDIC-Formatter.
pub struct EbcdicFormatter;

impl ByteFormatter for EbcdicFormatter {
    fn name(&self) -> &'static str { "EBCDIC" }

    fn parse(&mut self, buffer: &[u8]) -> Vec<Byte> {
        buffer.iter().map(|&byte| {
            let (category, character) = LOOKUP_EBCDIC[byte as usize];
            Byte{byte, category, character}
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::EbcdicFormatter;
    use super::ByteFormatter;

    #[test]
    fn name() {
        let formatter = EbcdicFormatter;
        assert_eq!("EBCDIC", formatter.name());
    }

    #[test]
    fn parse() {
        let mut formatter = EbcdicFormatter;
        let buffer = (0x00..=0x3f).map(|v| v).collect::<Vec<u8>>();
        assert_eq!(
            "0••••_••••••__•••••••••••••••••••••••_••••••••••××••••••••••••×•",
            formatter.parse(&buffer).iter().map(|character| {
                character.character.to_owned()
            }).collect::<Vec<String>>().join(""),
        );
        let buffer = (0x40..=0x7f).map(|v| v).collect::<Vec<u8>>();
        assert_eq!(
            " ×××××××××¢.<(+|&×××××××××!$*);¬-/××××××××¦,%_>?×××××××××`:#@'=\"",
            formatter.parse(&buffer).iter().map(|character| {
                character.character.to_owned()
            }).collect::<Vec<String>>().join(""),
        );
        let buffer = (0x80..=0xbf).map(|v| v).collect::<Vec<u8>>();
        assert_eq!(
            "×abcdefghi×××××±×jklmnopqr×××××××~stuvwxyz××××××^×××××××××[]××××",
            formatter.parse(&buffer).iter().map(|character| {
                character.character.to_owned()
            }).collect::<Vec<String>>().join(""),
        );
        let buffer = (0xc0..=0xff).map(|v| v).collect::<Vec<u8>>();
        assert_eq!(
            "{ABCDEFGHI××××××}JKLMNOPQR××××××\\×STUVWXYZ××××××0123456789×××××•",
            formatter.parse(&buffer).iter().map(|character| {
                character.character.to_owned()
            }).collect::<Vec<String>>().join(""),
        );
    }
}
