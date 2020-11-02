use core::iter::Iterator;
use super::{Byte, ByteCategory, ByteFormatter};

macro_rules! n {()                    => {(ByteCategory::Invalid,     "×"       )};}
macro_rules! o {()                    => {(ByteCategory::Control,     "•"       )};}
macro_rules! p {($Character:literal)  => {(ByteCategory::Printable,   $Character)};}
macro_rules! w {($Character:literal)  => {(ByteCategory::Whitespace,  $Character)};}
macro_rules! z {()                    => {(ByteCategory::Null,        "0"       )};}

pub(crate) const LOOKUP_ASCII: [(ByteCategory, &'static str); 256] = [
    z!(),     o!(),     o!(),     o!(),     o!(),     o!(),     o!(),     o!(),
    o!(),     w!("_" ), w!("_" ), o!(),     o!(),     o!(),     o!(),     o!(),
    o!(),     o!(),     o!(),     o!(),     o!(),     o!(),     o!(),     o!(),
    o!(),     o!(),     o!(),     o!(),     o!(),     o!(),     o!(),     o!(),
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
    p!("x" ), p!("y" ), p!("z" ), p!("{" ), p!("|" ), p!("}" ), p!("~" ), o!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
    n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),     n!(),
];

pub struct AsciiFormatter;

impl ByteFormatter for AsciiFormatter {
    fn parse(&mut self, buffer: &[u8]) -> Vec<Byte> {
        buffer
        .into_iter()
        .map(|&byte| {
            let item = LOOKUP_ASCII[byte as usize];
            Byte {
                byte,
                category: item.0,
                character: item.1,
            }
        })
        .collect()
    }
}
