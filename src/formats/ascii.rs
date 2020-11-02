use core::iter::Iterator;
use super::{Byte, ByteCategory, ByteFormatter};

macro_rules! c {()                    => {(ByteCategory::Control,     "•"       )};}
macro_rules! i {()                    => {(ByteCategory::Invalid,     "×"       )};}
macro_rules! n {()                    => {(ByteCategory::Null,        "0"       )};}
macro_rules! p {($Character:literal)  => {(ByteCategory::Printable,   $Character)};}
macro_rules! w {($Character:literal)  => {(ByteCategory::Whitespace,  $Character)};}

pub(crate) const LOOKUP_ASCII: [(ByteCategory, &str); 256] = [
    n!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),
    c!(),     w!("_" ), w!("_" ), c!(),     w!("_" ), w!("_" ), c!(),     c!(),
    c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),
    c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),     c!(),
    w!(" " ), p!("!" ), p!("\""), p!("#" ), p!("$" ), p!("%" ), p!("&" ), p!("\'"),
    p!("(" ), p!(")" ), p!("*" ), p!("+" ), p!("," ), p!("-" ), p!("." ), p!("/" ),
    p!("0" ), p!("1" ), p!("2" ), p!("3" ), p!("4" ), p!("5" ), p!("6" ), p!("7" ),
    p!("8" ), p!("9" ), p!(":" ), p!(";" ), p!("<" ), p!("=" ), p!(">" ), p!("?" ),
    p!("@" ), p!("A" ), p!("B" ), p!("C" ), p!("D" ), p!("E" ), p!("F" ), p!("G" ),
    p!("H" ), p!("I" ), p!("J" ), p!("K" ), p!("L" ), p!("M" ), p!("N" ), p!("O" ),
    p!("P" ), p!("Q" ), p!("R" ), p!("S" ), p!("T" ), p!("U" ), p!("V" ), p!("W" ),
    p!("X" ), p!("Y" ), p!("Z" ), p!("[" ), p!("\\"), p!("]" ), p!("^" ), p!("_" ),
    p!("`" ), p!("a" ), p!("b" ), p!("c" ), p!("d" ), p!("e" ), p!("f" ), p!("g" ),
    p!("h" ), p!("i" ), p!("j" ), p!("k" ), p!("l" ), p!("m" ), p!("n" ), p!("o" ),
    p!("p" ), p!("q" ), p!("r" ), p!("s" ), p!("t" ), p!("u" ), p!("v" ), p!("w" ),
    p!("x" ), p!("y" ), p!("z" ), p!("{" ), p!("|" ), p!("}" ), p!("~" ), c!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
    i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),     i!(),
];

/// The ASCII-Formatter.
pub struct AsciiFormatter;

impl ByteFormatter for AsciiFormatter {
    fn name(&self) -> &'static str { "ASCII" }

    fn parse(&mut self, buffer: &[u8]) -> Vec<Byte> {
        buffer.iter().map(|&byte| {
            let (category, character) = LOOKUP_ASCII[byte as usize];
            Byte{byte, category, character}
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::AsciiFormatter;
    use super::ByteFormatter;

    #[test]
    fn name() {
        let formatter = AsciiFormatter;
        assert_eq!("ASCII", formatter.name());
    }

    #[test]
    fn parse() {
        let mut formatter = AsciiFormatter;
        let buffer = (0x00..=0xff).map(|v| v).collect::<Vec<u8>>();
        assert_eq!(
            buffer.iter().map(|&byte|
                if      byte == 0x00                {'0'}
                else if byte == 0x20                {' '}
                else if byte.is_ascii_graphic()     {byte as char}
                else if byte.is_ascii_whitespace()  {'_'}
                else if byte.is_ascii()             {'•'}
                else                                {'×'}.to_string()
            ).collect::<Vec<String>>().join(""),
            formatter.parse(&buffer).iter().map(|character| {
                character.character.to_owned()
            }).collect::<Vec<String>>().join(""),
        )
    }
}
