use owo_colors::{colors, AnsiColors, Color, DynColors, OwoColorize};
use std::str::FromStr;
use std::sync::LazyLock;

pub static COLOR_NULL: LazyLock<String> =
    LazyLock::new(|| init_color("NULL", AnsiColors::BrightBlack));
pub static COLOR_OFFSET: LazyLock<String> =
    LazyLock::new(|| init_color("OFFSET", AnsiColors::BrightBlack));
pub static COLOR_ASCII_PRINTABLE: LazyLock<String> =
    LazyLock::new(|| init_color("ASCII_PRINTABLE", AnsiColors::Cyan));
pub static COLOR_ASCII_WHITESPACE: LazyLock<String> =
    LazyLock::new(|| init_color("ASCII_WHITESPACE", AnsiColors::Green));
pub static COLOR_ASCII_OTHER: LazyLock<String> =
    LazyLock::new(|| init_color("ASCII_OTHER", AnsiColors::Green));
pub static COLOR_NONASCII: LazyLock<String> =
    LazyLock::new(|| init_color("NONASCII", AnsiColors::Yellow));
pub const COLOR_RESET: &str = colors::Default::ANSI_FG;

fn init_color(name: &str, default_ansi: AnsiColors) -> String {
    let default = DynColors::Ansi(default_ansi);
    let env_var = format!("HEXYL_{}", name);
    let color = match std::env::var(env_var).as_deref() {
        Ok(color) => match DynColors::from_str(color) {
            Ok(color) => color,
            _ => default,
        },
        _ => default,
    };
    // owo_colors' API isn't designed to get the terminal codes directly for
    // dynamic colors, so we use this hack to get them from the LHS of some text.
    format!("{}", "|".color(color))
        .split_once("|")
        .unwrap()
        .0
        .to_owned()
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
