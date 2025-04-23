use owo_colors::{colors, Color};

pub const COLOR_NULL: &[u8] = colors::BrightBlack::ANSI_FG.as_bytes();
pub const COLOR_OFFSET: &[u8] = colors::BrightBlack::ANSI_FG.as_bytes();
pub const COLOR_ASCII_PRINTABLE: &[u8] = colors::Cyan::ANSI_FG.as_bytes();
pub const COLOR_ASCII_WHITESPACE: &[u8] = colors::Green::ANSI_FG.as_bytes();
pub const COLOR_ASCII_OTHER: &[u8] = colors::Green::ANSI_FG.as_bytes();
pub const COLOR_NONASCII: &[u8] = colors::Yellow::ANSI_FG.as_bytes();
pub const COLOR_RESET: &[u8] = colors::Default::ANSI_FG.as_bytes();

pub const COLOR_NULL_RGB: &[u8] = &rgb_bytes(100, 100, 100);

pub const COLOR_DEL: &[u8] = &rgb_bytes(64, 128, 0);

pub const COLOR_GRADIENT_NONASCII: [[u8; 19]; 128] =
    generate_color_gradient(&[(255, 0, 0, 0.0), (255, 255, 0, 0.66), (255, 255, 255, 1.0)]);

pub const COLOR_GRADIENT_ASCII_NONPRINTABLE: [[u8; 19]; 31] =
    generate_color_gradient(&[(255, 0, 255, 0.0), (128, 0, 255, 1.0)]);

pub const COLOR_GRADIENT_ASCII_PRINTABLE: [[u8; 19]; 95] =
    generate_color_gradient(&[(0, 128, 255, 0.0), (0, 255, 128, 1.0)]);

const fn as_dec(byte: u8) -> [u8; 3] {
    [
        b'0' + (byte / 100),
        b'0' + ((byte % 100) / 10),
        b'0' + (byte % 10),
    ]
}

const fn rgb_bytes(r: u8, g: u8, b: u8) -> [u8; 19] {
    let mut buf = *b"\x1b[38;2;rrr;ggg;bbbm";

    // r 7
    buf[7] = as_dec(r)[0];
    buf[8] = as_dec(r)[1];
    buf[9] = as_dec(r)[2];

    // g 11
    buf[11] = as_dec(g)[0];
    buf[12] = as_dec(g)[1];
    buf[13] = as_dec(g)[2];

    // b 15
    buf[15] = as_dec(b)[0];
    buf[16] = as_dec(b)[1];
    buf[17] = as_dec(b)[2];

    buf
}

const fn generate_color_gradient<const N: usize>(stops: &[(u8, u8, u8, f64)]) -> [[u8; 19]; N] {
    let mut out = [rgb_bytes(0, 0, 0); N];

    assert!(stops.len() >= 2, "need at least two stops for the gradient");

    let mut byte = 0;
    while byte < N {
        let relative_byte = byte as f64 / N as f64;

        let mut i = 1;
        while i < stops.len() && stops[i].3 < relative_byte {
            i += 1;
        }
        if i >= stops.len() {
            i = stops.len() - 1;
        }
        let prev_stop = stops[i - 1];
        let stop = stops[i];
        let diff = stop.3 - prev_stop.3;
        let t = (relative_byte - prev_stop.3) / diff;

        let r = (prev_stop.0 as f64 + (t * (stop.0 as f64 - prev_stop.0 as f64))) as u8;
        let g = (prev_stop.1 as f64 + (t * (stop.1 as f64 - prev_stop.1 as f64))) as u8;
        let b = (prev_stop.2 as f64 + (t * (stop.2 as f64 - prev_stop.2 as f64))) as u8;

        out[byte] = rgb_bytes(r, g, b);

        byte += 1;
    }

    out
}

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

#[rustfmt::skip]
pub const CP1047: [char; 256] = [
     //
     //  Copyright (c) 2016,2024 IBM Corporation and other Contributors.
     //
     //  All rights reserved. This program and the accompanying materials
     //  are made available under the terms of the Eclipse Public License v1.0
     //  which accompanies this distribution, and is available at
     //  http://www.eclipse.org/legal/epl-v10.html
     //
     //  Contributors:
     //    Mark Taylor - Initial Contribution
     //

     // ref1 https://github.com/ibm-messaging/mq-smf-csv/blob/master/src/smfConv.c
    //  ref2 https://web.archive.org/web/20150607033635/http://www-01.ibm.com/software/globalization/cp/cp01047.html
    '.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.',
    '.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.',
    '.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.',
    '.','.','.','.','.','.','.','.','.','.','.','.','.','.','.','.',
    ' ','.','.','.','.','.','.','.','.','.','$','.','<','(','+','|',
    '&','.','.','.','.','.','.','.','.','.','!','$','*',')',';','.',
    '-','/','.','.','.','.','.','.','.','.','.',',','%','_','>','?',
    '.','.','.','.','.','.','.','.','.','.',':','#','@','\'','=','.',
    '.','a','b','c','d','e','f','g','h','i','.','{','.','(','+','.',
    '.','j','k','l','m','n','o','p','q','r','.','}','.',')','.','.',
    '.','~','s','t','u','v','w','x','y','z','.','.','.','.','.','.',
    '.','.','.','.','.','.','.','.','.','.','[',']','.','.','.','-',
    '{','A','B','C','D','E','F','G','H','I','.','.','.','.','.','.',
    '}','J','K','L','M','N','O','P','Q','R','.','.','.','.','.','.',
    '.','.','S','T','U','V','W','X','Y','Z','.','.','.','.','.','.',
    '0','1','2','3','4','5','6','7','8','9','.','.','.','.','.','.'
];
