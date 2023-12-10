use owo_colors::{colors, Color};

pub const COLOR_NULL: &[u8] = colors::BrightBlack::ANSI_FG.as_bytes();
pub const COLOR_OFFSET: &[u8] = colors::BrightBlack::ANSI_FG.as_bytes();
pub const COLOR_ASCII_PRINTABLE: &[u8] = colors::Cyan::ANSI_FG.as_bytes();
pub const COLOR_ASCII_WHITESPACE: &[u8] = colors::Green::ANSI_FG.as_bytes();
pub const COLOR_ASCII_OTHER: &[u8] = colors::Green::ANSI_FG.as_bytes();
pub const COLOR_NONASCII: &[u8] = colors::Yellow::ANSI_FG.as_bytes();
pub const COLOR_RESET: &[u8] = colors::Default::ANSI_FG.as_bytes();

#[rustfmt::skip]
pub const CP437: [char; 256] = [
    // Copyright (c) 2016, Delan Azabani <delan@azabani.com>
    //
    // Permission to use, copy, modify, and/or distribute this software for any
    // purpose with or without fee is hereby granted, provided that the above
    // copyright notice and this permission notice appear in all copies.
    //
    // THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
    // WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
    // MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
    // ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
    // WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
    // ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
    // OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
    //
    // modified to use the ⋄ character instead of ␀

    // use https://en.wikipedia.org/w/index.php?title=Code_page_437&oldid=978947122
    // not ftp://ftp.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT
    // because we want the graphic versions of 01h–1Fh + 7Fh
    '⋄','☺','☻','♥','♦','♣','♠','•','◘','○','◙','♂','♀','♪','♫','☼',
    '►','◄','↕','‼','¶','§','▬','↨','↑','↓','→','←','∟','↔','▲','▼',
    ' ','!','"','#','$','%','&','\'','(',')','*','+',',','-','.','/',
    '0','1','2','3','4','5','6','7','8','9',':',';','<','=','>','?',
    '@','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O',
    'P','Q','R','S','T','U','V','W','X','Y','Z','[','\\',']','^','_',
    '`','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o',
    'p','q','r','s','t','u','v','w','x','y','z','{','|','}','~','⌂',
    'Ç','ü','é','â','ä','à','å','ç','ê','ë','è','ï','î','ì','Ä','Å',
    'É','æ','Æ','ô','ö','ò','û','ù','ÿ','Ö','Ü','¢','£','¥','₧','ƒ',
    'á','í','ó','ú','ñ','Ñ','ª','º','¿','⌐','¬','½','¼','¡','«','»',
    '░','▒','▓','│','┤','╡','╢','╖','╕','╣','║','╗','╝','╜','╛','┐',
    '└','┴','┬','├','─','┼','╞','╟','╚','╔','╩','╦','╠','═','╬','╧',
    '╨','╤','╥','╙','╘','╒','╓','╫','╪','┘','┌','█','▄','▌','▐','▀',
    'α','ß','Γ','π','Σ','σ','µ','τ','Φ','Θ','Ω','δ','∞','φ','ε','∩',
    '≡','±','≥','≤','⌠','⌡','÷','≈','°','∙','·','√','ⁿ','²','■','ﬀ',
];
