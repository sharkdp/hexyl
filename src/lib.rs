pub(crate) mod input;

pub use input::*;

use std::io::{self, BufReader, Read, Write};

use owo_colors::{colors, Color};

pub enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

const COLOR_NULL: &[u8] = colors::BrightBlack::ANSI_FG.as_bytes();
const COLOR_OFFSET: &[u8] = colors::BrightBlack::ANSI_FG.as_bytes();
const COLOR_ASCII_PRINTABLE: &[u8] = colors::Cyan::ANSI_FG.as_bytes();
const COLOR_ASCII_WHITESPACE: &[u8] = colors::Green::ANSI_FG.as_bytes();
const COLOR_ASCII_OTHER: &[u8] = colors::Green::ANSI_FG.as_bytes();
const COLOR_NONASCII: &[u8] = colors::Yellow::ANSI_FG.as_bytes();
const COLOR_RESET: &[u8] = colors::Default::ANSI_FG.as_bytes();
const COLORS_XTERM: [&[u8]; 256] = [
    colors::xterm::UserBlack::ANSI_FG.as_bytes(),
    colors::xterm::UserRed::ANSI_FG.as_bytes(),
    colors::xterm::UserGreen::ANSI_FG.as_bytes(),
    colors::xterm::UserYellow::ANSI_FG.as_bytes(),
    colors::xterm::UserBlue::ANSI_FG.as_bytes(),
    colors::xterm::UserMagenta::ANSI_FG.as_bytes(),
    colors::xterm::UserCyan::ANSI_FG.as_bytes(),
    colors::xterm::UserWhite::ANSI_FG.as_bytes(),
    colors::xterm::UserBrightBlack::ANSI_FG.as_bytes(),
    colors::xterm::UserBrightRed::ANSI_FG.as_bytes(),
    colors::xterm::UserBrightGreen::ANSI_FG.as_bytes(),
    colors::xterm::UserBrightYellow::ANSI_FG.as_bytes(),
    colors::xterm::UserBrightBlue::ANSI_FG.as_bytes(),
    colors::xterm::UserBrightMagenta::ANSI_FG.as_bytes(),
    colors::xterm::UserBrightCyan::ANSI_FG.as_bytes(),
    colors::xterm::UserBrightWhite::ANSI_FG.as_bytes(),
    colors::xterm::Black::ANSI_FG.as_bytes(),
    colors::xterm::StratosBlue::ANSI_FG.as_bytes(),
    colors::xterm::NavyBlue::ANSI_FG.as_bytes(),
    colors::xterm::MidnightBlue::ANSI_FG.as_bytes(),
    colors::xterm::DarkBlue::ANSI_FG.as_bytes(),
    colors::xterm::Blue::ANSI_FG.as_bytes(),
    colors::xterm::CamaroneGreen::ANSI_FG.as_bytes(),
    colors::xterm::BlueStone::ANSI_FG.as_bytes(),
    colors::xterm::OrientBlue::ANSI_FG.as_bytes(),
    colors::xterm::EndeavourBlue::ANSI_FG.as_bytes(),
    colors::xterm::ScienceBlue::ANSI_FG.as_bytes(),
    colors::xterm::BlueRibbon::ANSI_FG.as_bytes(),
    colors::xterm::JapaneseLaurel::ANSI_FG.as_bytes(),
    colors::xterm::DeepSeaGreen::ANSI_FG.as_bytes(),
    colors::xterm::Teal::ANSI_FG.as_bytes(),
    colors::xterm::DeepCerulean::ANSI_FG.as_bytes(),
    colors::xterm::LochmaraBlue::ANSI_FG.as_bytes(),
    colors::xterm::AzureRadiance::ANSI_FG.as_bytes(),
    colors::xterm::LightJapaneseLaurel::ANSI_FG.as_bytes(),
    colors::xterm::Jade::ANSI_FG.as_bytes(),
    colors::xterm::PersianGreen::ANSI_FG.as_bytes(),
    colors::xterm::BondiBlue::ANSI_FG.as_bytes(),
    colors::xterm::Cerulean::ANSI_FG.as_bytes(),
    colors::xterm::LightAzureRadiance::ANSI_FG.as_bytes(),
    colors::xterm::DarkGreen::ANSI_FG.as_bytes(),
    colors::xterm::Malachite::ANSI_FG.as_bytes(),
    colors::xterm::CaribbeanGreen::ANSI_FG.as_bytes(),
    colors::xterm::LightCaribbeanGreen::ANSI_FG.as_bytes(),
    colors::xterm::RobinEggBlue::ANSI_FG.as_bytes(),
    colors::xterm::Aqua::ANSI_FG.as_bytes(),
    colors::xterm::Green::ANSI_FG.as_bytes(),
    colors::xterm::DarkSpringGreen::ANSI_FG.as_bytes(),
    colors::xterm::SpringGreen::ANSI_FG.as_bytes(),
    colors::xterm::LightSpringGreen::ANSI_FG.as_bytes(),
    colors::xterm::BrightTurquoise::ANSI_FG.as_bytes(),
    colors::xterm::Cyan::ANSI_FG.as_bytes(),
    colors::xterm::Rosewood::ANSI_FG.as_bytes(),
    colors::xterm::PompadourMagenta::ANSI_FG.as_bytes(),
    colors::xterm::PigmentIndigo::ANSI_FG.as_bytes(),
    colors::xterm::DarkPurple::ANSI_FG.as_bytes(),
    colors::xterm::ElectricIndigo::ANSI_FG.as_bytes(),
    colors::xterm::ElectricPurple::ANSI_FG.as_bytes(),
    colors::xterm::VerdunGreen::ANSI_FG.as_bytes(),
    colors::xterm::ScorpionOlive::ANSI_FG.as_bytes(),
    colors::xterm::Lilac::ANSI_FG.as_bytes(),
    colors::xterm::ScampiIndigo::ANSI_FG.as_bytes(),
    colors::xterm::Indigo::ANSI_FG.as_bytes(),
    colors::xterm::DarkCornflowerBlue::ANSI_FG.as_bytes(),
    colors::xterm::DarkLimeade::ANSI_FG.as_bytes(),
    colors::xterm::GladeGreen::ANSI_FG.as_bytes(),
    colors::xterm::JuniperGreen::ANSI_FG.as_bytes(),
    colors::xterm::HippieBlue::ANSI_FG.as_bytes(),
    colors::xterm::HavelockBlue::ANSI_FG.as_bytes(),
    colors::xterm::CornflowerBlue::ANSI_FG.as_bytes(),
    colors::xterm::Limeade::ANSI_FG.as_bytes(),
    colors::xterm::FernGreen::ANSI_FG.as_bytes(),
    colors::xterm::SilverTree::ANSI_FG.as_bytes(),
    colors::xterm::Tradewind::ANSI_FG.as_bytes(),
    colors::xterm::ShakespeareBlue::ANSI_FG.as_bytes(),
    colors::xterm::DarkMalibuBlue::ANSI_FG.as_bytes(),
    colors::xterm::DarkBrightGreen::ANSI_FG.as_bytes(),
    colors::xterm::DarkPastelGreen::ANSI_FG.as_bytes(),
    colors::xterm::PastelGreen::ANSI_FG.as_bytes(),
    colors::xterm::DownyTeal::ANSI_FG.as_bytes(),
    colors::xterm::Viking::ANSI_FG.as_bytes(),
    colors::xterm::MalibuBlue::ANSI_FG.as_bytes(),
    colors::xterm::BrightGreen::ANSI_FG.as_bytes(),
    colors::xterm::DarkScreaminGreen::ANSI_FG.as_bytes(),
    colors::xterm::ScreaminGreen::ANSI_FG.as_bytes(),
    colors::xterm::DarkAquamarine::ANSI_FG.as_bytes(),
    colors::xterm::Aquamarine::ANSI_FG.as_bytes(),
    colors::xterm::LightAquamarine::ANSI_FG.as_bytes(),
    colors::xterm::Maroon::ANSI_FG.as_bytes(),
    colors::xterm::DarkFreshEggplant::ANSI_FG.as_bytes(),
    colors::xterm::LightFreshEggplant::ANSI_FG.as_bytes(),
    colors::xterm::Purple::ANSI_FG.as_bytes(),
    colors::xterm::ElectricViolet::ANSI_FG.as_bytes(),
    colors::xterm::LightElectricViolet::ANSI_FG.as_bytes(),
    colors::xterm::Brown::ANSI_FG.as_bytes(),
    colors::xterm::CopperRose::ANSI_FG.as_bytes(),
    colors::xterm::StrikemasterPurple::ANSI_FG.as_bytes(),
    colors::xterm::DelugePurple::ANSI_FG.as_bytes(),
    colors::xterm::DarkMediumPurple::ANSI_FG.as_bytes(),
    colors::xterm::DarkHeliotropePurple::ANSI_FG.as_bytes(),
    colors::xterm::Olive::ANSI_FG.as_bytes(),
    colors::xterm::ClayCreekOlive::ANSI_FG.as_bytes(),
    colors::xterm::DarkGray::ANSI_FG.as_bytes(),
    colors::xterm::WildBlueYonder::ANSI_FG.as_bytes(),
    colors::xterm::ChetwodeBlue::ANSI_FG.as_bytes(),
    colors::xterm::SlateBlue::ANSI_FG.as_bytes(),
    colors::xterm::LightLimeade::ANSI_FG.as_bytes(),
    colors::xterm::ChelseaCucumber::ANSI_FG.as_bytes(),
    colors::xterm::BayLeaf::ANSI_FG.as_bytes(),
    colors::xterm::GulfStream::ANSI_FG.as_bytes(),
    colors::xterm::PoloBlue::ANSI_FG.as_bytes(),
    colors::xterm::LightMalibuBlue::ANSI_FG.as_bytes(),
    colors::xterm::Pistachio::ANSI_FG.as_bytes(),
    colors::xterm::LightPastelGreen::ANSI_FG.as_bytes(),
    colors::xterm::DarkFeijoaGreen::ANSI_FG.as_bytes(),
    colors::xterm::VistaBlue::ANSI_FG.as_bytes(),
    colors::xterm::Bermuda::ANSI_FG.as_bytes(),
    colors::xterm::DarkAnakiwaBlue::ANSI_FG.as_bytes(),
    colors::xterm::ChartreuseGreen::ANSI_FG.as_bytes(),
    colors::xterm::LightScreaminGreen::ANSI_FG.as_bytes(),
    colors::xterm::DarkMintGreen::ANSI_FG.as_bytes(),
    colors::xterm::MintGreen::ANSI_FG.as_bytes(),
    colors::xterm::LighterAquamarine::ANSI_FG.as_bytes(),
    colors::xterm::AnakiwaBlue::ANSI_FG.as_bytes(),
    colors::xterm::BrightRed::ANSI_FG.as_bytes(),
    colors::xterm::DarkFlirt::ANSI_FG.as_bytes(),
    colors::xterm::Flirt::ANSI_FG.as_bytes(),
    colors::xterm::LightFlirt::ANSI_FG.as_bytes(),
    colors::xterm::DarkViolet::ANSI_FG.as_bytes(),
    colors::xterm::BrightElectricViolet::ANSI_FG.as_bytes(),
    colors::xterm::RoseofSharonOrange::ANSI_FG.as_bytes(),
    colors::xterm::MatrixPink::ANSI_FG.as_bytes(),
    colors::xterm::TapestryPink::ANSI_FG.as_bytes(),
    colors::xterm::FuchsiaPink::ANSI_FG.as_bytes(),
    colors::xterm::MediumPurple::ANSI_FG.as_bytes(),
    colors::xterm::Heliotrope::ANSI_FG.as_bytes(),
    colors::xterm::PirateGold::ANSI_FG.as_bytes(),
    colors::xterm::MuesliOrange::ANSI_FG.as_bytes(),
    colors::xterm::PharlapPink::ANSI_FG.as_bytes(),
    colors::xterm::Bouquet::ANSI_FG.as_bytes(),
    colors::xterm::Lavender::ANSI_FG.as_bytes(),
    colors::xterm::LightHeliotrope::ANSI_FG.as_bytes(),
    colors::xterm::BuddhaGold::ANSI_FG.as_bytes(),
    colors::xterm::OliveGreen::ANSI_FG.as_bytes(),
    colors::xterm::HillaryOlive::ANSI_FG.as_bytes(),
    colors::xterm::SilverChalice::ANSI_FG.as_bytes(),
    colors::xterm::WistfulLilac::ANSI_FG.as_bytes(),
    colors::xterm::MelroseLilac::ANSI_FG.as_bytes(),
    colors::xterm::RioGrandeGreen::ANSI_FG.as_bytes(),
    colors::xterm::ConiferGreen::ANSI_FG.as_bytes(),
    colors::xterm::Feijoa::ANSI_FG.as_bytes(),
    colors::xterm::PixieGreen::ANSI_FG.as_bytes(),
    colors::xterm::JungleMist::ANSI_FG.as_bytes(),
    colors::xterm::LightAnakiwaBlue::ANSI_FG.as_bytes(),
    colors::xterm::Lime::ANSI_FG.as_bytes(),
    colors::xterm::GreenYellow::ANSI_FG.as_bytes(),
    colors::xterm::LightMintGreen::ANSI_FG.as_bytes(),
    colors::xterm::Celadon::ANSI_FG.as_bytes(),
    colors::xterm::AeroBlue::ANSI_FG.as_bytes(),
    colors::xterm::FrenchPassLightBlue::ANSI_FG.as_bytes(),
    colors::xterm::GuardsmanRed::ANSI_FG.as_bytes(),
    colors::xterm::RazzmatazzCerise::ANSI_FG.as_bytes(),
    colors::xterm::MediumVioletRed::ANSI_FG.as_bytes(),
    colors::xterm::HollywoodCerise::ANSI_FG.as_bytes(),
    colors::xterm::DarkPurplePizzazz::ANSI_FG.as_bytes(),
    colors::xterm::BrighterElectricViolet::ANSI_FG.as_bytes(),
    colors::xterm::TennOrange::ANSI_FG.as_bytes(),
    colors::xterm::RomanOrange::ANSI_FG.as_bytes(),
    colors::xterm::CranberryPink::ANSI_FG.as_bytes(),
    colors::xterm::HopbushPink::ANSI_FG.as_bytes(),
    colors::xterm::Orchid::ANSI_FG.as_bytes(),
    colors::xterm::LighterHeliotrope::ANSI_FG.as_bytes(),
    colors::xterm::MangoTango::ANSI_FG.as_bytes(),
    colors::xterm::Copperfield::ANSI_FG.as_bytes(),
    colors::xterm::SeaPink::ANSI_FG.as_bytes(),
    colors::xterm::CanCanPink::ANSI_FG.as_bytes(),
    colors::xterm::LightOrchid::ANSI_FG.as_bytes(),
    colors::xterm::BrightHeliotrope::ANSI_FG.as_bytes(),
    colors::xterm::DarkCorn::ANSI_FG.as_bytes(),
    colors::xterm::DarkTachaOrange::ANSI_FG.as_bytes(),
    colors::xterm::TanBeige::ANSI_FG.as_bytes(),
    colors::xterm::ClamShell::ANSI_FG.as_bytes(),
    colors::xterm::ThistlePink::ANSI_FG.as_bytes(),
    colors::xterm::Mauve::ANSI_FG.as_bytes(),
    colors::xterm::Corn::ANSI_FG.as_bytes(),
    colors::xterm::TachaOrange::ANSI_FG.as_bytes(),
    colors::xterm::DecoOrange::ANSI_FG.as_bytes(),
    colors::xterm::PaleGoldenrod::ANSI_FG.as_bytes(),
    colors::xterm::AltoBeige::ANSI_FG.as_bytes(),
    colors::xterm::FogPink::ANSI_FG.as_bytes(),
    colors::xterm::ChartreuseYellow::ANSI_FG.as_bytes(),
    colors::xterm::Canary::ANSI_FG.as_bytes(),
    colors::xterm::Honeysuckle::ANSI_FG.as_bytes(),
    colors::xterm::ReefPaleYellow::ANSI_FG.as_bytes(),
    colors::xterm::SnowyMint::ANSI_FG.as_bytes(),
    colors::xterm::OysterBay::ANSI_FG.as_bytes(),
    colors::xterm::Red::ANSI_FG.as_bytes(),
    colors::xterm::DarkRose::ANSI_FG.as_bytes(),
    colors::xterm::Rose::ANSI_FG.as_bytes(),
    colors::xterm::LightHollywoodCerise::ANSI_FG.as_bytes(),
    colors::xterm::PurplePizzazz::ANSI_FG.as_bytes(),
    colors::xterm::Fuchsia::ANSI_FG.as_bytes(),
    colors::xterm::BlazeOrange::ANSI_FG.as_bytes(),
    colors::xterm::BittersweetOrange::ANSI_FG.as_bytes(),
    colors::xterm::WildWatermelon::ANSI_FG.as_bytes(),
    colors::xterm::DarkHotPink::ANSI_FG.as_bytes(),
    colors::xterm::HotPink::ANSI_FG.as_bytes(),
    colors::xterm::PinkFlamingo::ANSI_FG.as_bytes(),
    colors::xterm::FlushOrange::ANSI_FG.as_bytes(),
    colors::xterm::Salmon::ANSI_FG.as_bytes(),
    colors::xterm::VividTangerine::ANSI_FG.as_bytes(),
    colors::xterm::PinkSalmon::ANSI_FG.as_bytes(),
    colors::xterm::DarkLavenderRose::ANSI_FG.as_bytes(),
    colors::xterm::BlushPink::ANSI_FG.as_bytes(),
    colors::xterm::YellowSea::ANSI_FG.as_bytes(),
    colors::xterm::TexasRose::ANSI_FG.as_bytes(),
    colors::xterm::Tacao::ANSI_FG.as_bytes(),
    colors::xterm::Sundown::ANSI_FG.as_bytes(),
    colors::xterm::CottonCandy::ANSI_FG.as_bytes(),
    colors::xterm::LavenderRose::ANSI_FG.as_bytes(),
    colors::xterm::Gold::ANSI_FG.as_bytes(),
    colors::xterm::Dandelion::ANSI_FG.as_bytes(),
    colors::xterm::GrandisCaramel::ANSI_FG.as_bytes(),
    colors::xterm::Caramel::ANSI_FG.as_bytes(),
    colors::xterm::CosmosSalmon::ANSI_FG.as_bytes(),
    colors::xterm::PinkLace::ANSI_FG.as_bytes(),
    colors::xterm::Yellow::ANSI_FG.as_bytes(),
    colors::xterm::LaserLemon::ANSI_FG.as_bytes(),
    colors::xterm::DollyYellow::ANSI_FG.as_bytes(),
    colors::xterm::PortafinoYellow::ANSI_FG.as_bytes(),
    colors::xterm::Cumulus::ANSI_FG.as_bytes(),
    colors::xterm::White::ANSI_FG.as_bytes(),
    colors::xterm::DarkCodGray::ANSI_FG.as_bytes(),
    colors::xterm::CodGray::ANSI_FG.as_bytes(),
    colors::xterm::LightCodGray::ANSI_FG.as_bytes(),
    colors::xterm::DarkMineShaft::ANSI_FG.as_bytes(),
    colors::xterm::MineShaft::ANSI_FG.as_bytes(),
    colors::xterm::LightMineShaft::ANSI_FG.as_bytes(),
    colors::xterm::DarkTundora::ANSI_FG.as_bytes(),
    colors::xterm::Tundora::ANSI_FG.as_bytes(),
    colors::xterm::ScorpionGray::ANSI_FG.as_bytes(),
    colors::xterm::DarkDoveGray::ANSI_FG.as_bytes(),
    colors::xterm::DoveGray::ANSI_FG.as_bytes(),
    colors::xterm::Boulder::ANSI_FG.as_bytes(),
    colors::xterm::Gray::ANSI_FG.as_bytes(),
    colors::xterm::LightGray::ANSI_FG.as_bytes(),
    colors::xterm::DustyGray::ANSI_FG.as_bytes(),
    colors::xterm::NobelGray::ANSI_FG.as_bytes(),
    colors::xterm::DarkSilverChalice::ANSI_FG.as_bytes(),
    colors::xterm::LightSilverChalice::ANSI_FG.as_bytes(),
    colors::xterm::DarkSilver::ANSI_FG.as_bytes(),
    colors::xterm::Silver::ANSI_FG.as_bytes(),
    colors::xterm::DarkAlto::ANSI_FG.as_bytes(),
    colors::xterm::Alto::ANSI_FG.as_bytes(),
    colors::xterm::Mercury::ANSI_FG.as_bytes(),
    colors::xterm::GalleryGray::ANSI_FG.as_bytes(),
];

#[rustfmt::skip]
const CP437: [char; 256] = [
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

#[derive(Copy, Clone)]
pub enum ByteCategory {
    Null,
    AsciiPrintable,
    AsciiWhitespace,
    AsciiOther,
    NonAscii,
}

#[derive(Copy, Clone)]
#[non_exhaustive]
pub enum CharacterTable {
    AsciiOnly,
    Block,
    CP437,
}

#[derive(Copy, Clone)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(PartialEq)]
enum Squeezer {
    Print,
    Delete,
    Ignore,
    Disabled,
}

#[derive(Copy, Clone)]
struct Byte(u8);

impl Byte {
    fn category(self) -> ByteCategory {
        if self.0 == 0x00 {
            ByteCategory::Null
        } else if self.0.is_ascii_graphic() {
            ByteCategory::AsciiPrintable
        } else if self.0.is_ascii_whitespace() {
            ByteCategory::AsciiWhitespace
        } else if self.0.is_ascii() {
            ByteCategory::AsciiOther
        } else {
            ByteCategory::NonAscii
        }
    }

    fn color(self, character_table: CharacterTable) -> &'static [u8] {
        use crate::ByteCategory::*;
        match character_table {
            CharacterTable::AsciiOnly | CharacterTable::CP437 => match self.category() {
                Null => COLOR_NULL,
                AsciiPrintable => COLOR_ASCII_PRINTABLE,
                AsciiWhitespace => COLOR_ASCII_WHITESPACE,
                AsciiOther => COLOR_ASCII_OTHER,
                NonAscii => COLOR_NONASCII,
            },
            CharacterTable::Block => COLORS_XTERM[self.0 as usize],
        }
    }

    fn as_char(self, character_table: CharacterTable) -> char {
        use crate::ByteCategory::*;
        match character_table {
            CharacterTable::AsciiOnly => match self.category() {
                Null => '⋄',
                AsciiPrintable => self.0 as char,
                AsciiWhitespace if self.0 == 0x20 => ' ',
                AsciiWhitespace => '_',
                AsciiOther => '•',
                NonAscii => '×',
            },
            CharacterTable::CP437 => CP437[self.0 as usize],
            CharacterTable::Block => '█',
        }
    }
}

struct BorderElements {
    left_corner: char,
    horizontal_line: char,
    column_separator: char,
    right_corner: char,
}

#[derive(Clone, Copy)]
pub enum BorderStyle {
    Unicode,
    Ascii,
    None,
}

impl BorderStyle {
    fn header_elems(&self) -> Option<BorderElements> {
        match self {
            BorderStyle::Unicode => Some(BorderElements {
                left_corner: '┌',
                horizontal_line: '─',
                column_separator: '┬',
                right_corner: '┐',
            }),
            BorderStyle::Ascii => Some(BorderElements {
                left_corner: '+',
                horizontal_line: '-',
                column_separator: '+',
                right_corner: '+',
            }),
            BorderStyle::None => None,
        }
    }

    fn footer_elems(&self) -> Option<BorderElements> {
        match self {
            BorderStyle::Unicode => Some(BorderElements {
                left_corner: '└',
                horizontal_line: '─',
                column_separator: '┴',
                right_corner: '┘',
            }),
            BorderStyle::Ascii => Some(BorderElements {
                left_corner: '+',
                horizontal_line: '-',
                column_separator: '+',
                right_corner: '+',
            }),
            BorderStyle::None => None,
        }
    }

    fn outer_sep(&self) -> char {
        match self {
            BorderStyle::Unicode => '│',
            BorderStyle::Ascii => '|',
            BorderStyle::None => ' ',
        }
    }

    fn inner_sep(&self) -> char {
        match self {
            BorderStyle::Unicode => '┊',
            BorderStyle::Ascii => '|',
            BorderStyle::None => ' ',
        }
    }
}

pub struct PrinterBuilder<'a, Writer: Write> {
    writer: &'a mut Writer,
    show_color: bool,
    show_char_panel: bool,
    show_position_panel: bool,
    border_style: BorderStyle,
    use_squeeze: bool,
    panels: u64,
    group_size: u8,
    base: Base,
    endianness: Endianness,
    character_table: CharacterTable,
}

impl<'a, Writer: Write> PrinterBuilder<'a, Writer> {
    pub fn new(writer: &'a mut Writer) -> Self {
        PrinterBuilder {
            writer,
            show_color: true,
            show_char_panel: true,
            show_position_panel: true,
            border_style: BorderStyle::Unicode,
            use_squeeze: true,
            panels: 2,
            group_size: 1,
            base: Base::Hexadecimal,
            endianness: Endianness::Big,
            character_table: CharacterTable::AsciiOnly,
        }
    }

    pub fn show_color(mut self, show_color: bool) -> Self {
        self.show_color = show_color;
        self
    }

    pub fn show_char_panel(mut self, show_char_panel: bool) -> Self {
        self.show_char_panel = show_char_panel;
        self
    }

    pub fn show_position_panel(mut self, show_position_panel: bool) -> Self {
        self.show_position_panel = show_position_panel;
        self
    }

    pub fn with_border_style(mut self, border_style: BorderStyle) -> Self {
        self.border_style = border_style;
        self
    }

    pub fn enable_squeezing(mut self, enable: bool) -> Self {
        self.use_squeeze = enable;
        self
    }

    pub fn num_panels(mut self, num: u64) -> Self {
        self.panels = num;
        self
    }

    pub fn group_size(mut self, num: u8) -> Self {
        self.group_size = num;
        self
    }

    pub fn with_base(mut self, base: Base) -> Self {
        self.base = base;
        self
    }

    pub fn endianness(mut self, endianness: Endianness) -> Self {
        self.endianness = endianness;
        self
    }

    pub fn character_table(mut self, character_table: CharacterTable) -> Self {
        self.character_table = character_table;
        self
    }

    pub fn build(self) -> Printer<'a, Writer> {
        Printer::new(
            self.writer,
            self.show_color,
            self.show_char_panel,
            self.show_position_panel,
            self.border_style,
            self.use_squeeze,
            self.panels,
            self.group_size,
            self.base,
            self.endianness,
            self.character_table,
        )
    }
}

pub struct Printer<'a, Writer: Write> {
    idx: u64,
    /// the buffer containing all the bytes in a line for character printing
    line_buf: Vec<u8>,
    writer: &'a mut Writer,
    show_char_panel: bool,
    show_position_panel: bool,
    show_color: bool,
    curr_color: Option<&'static [u8]>,
    border_style: BorderStyle,
    byte_hex_panel: Vec<String>,
    byte_char_panel: Vec<String>,
    // same as previous but in Fixed(242) gray color, for position panel
    byte_hex_panel_g: Vec<String>,
    squeezer: Squeezer,
    display_offset: u64,
    /// The number of panels to draw.
    panels: u64,
    squeeze_byte: usize,
    /// The number of octets per group.
    group_size: u8,
    /// The number of digits used to write the base.
    base_digits: u8,
    /// Whether to show groups in little or big endian format.
    endianness: Endianness,
    /// The character table to reference for the character panel.
    character_table: CharacterTable,
}

impl<'a, Writer: Write> Printer<'a, Writer> {
    fn new(
        writer: &'a mut Writer,
        show_color: bool,
        show_char_panel: bool,
        show_position_panel: bool,
        border_style: BorderStyle,
        use_squeeze: bool,
        panels: u64,
        group_size: u8,
        base: Base,
        endianness: Endianness,
        character_table: CharacterTable,
    ) -> Printer<'a, Writer> {
        Printer {
            idx: 0,
            line_buf: vec![0x0; 8 * panels as usize],
            writer,
            show_char_panel,
            show_position_panel,
            show_color,
            curr_color: None,
            border_style,
            byte_hex_panel: (0u8..=u8::MAX)
                .map(|i| match base {
                    Base::Binary => format!("{i:08b}"),
                    Base::Octal => format!("{i:03o}"),
                    Base::Decimal => format!("{i:03}"),
                    Base::Hexadecimal => format!("{i:02x}"),
                })
                .collect(),
            byte_char_panel: (0u8..=u8::MAX)
                .map(|i| format!("{}", Byte(i).as_char(character_table)))
                .collect(),
            byte_hex_panel_g: (0u8..=u8::MAX).map(|i| format!("{i:02x}")).collect(),
            squeezer: if use_squeeze {
                Squeezer::Ignore
            } else {
                Squeezer::Disabled
            },
            display_offset: 0,
            panels,
            squeeze_byte: 0x00,
            group_size,
            base_digits: match base {
                Base::Binary => 8,
                Base::Octal => 3,
                Base::Decimal => 3,
                Base::Hexadecimal => 2,
            },
            endianness,
            character_table,
        }
    }

    pub fn display_offset(&mut self, display_offset: u64) -> &mut Self {
        self.display_offset = display_offset;
        self
    }

    fn panel_sz(&self) -> usize {
        // add one to include the trailing space of a group
        let group_sz = self.base_digits as usize * self.group_size as usize + 1;
        let group_per_panel = 8 / self.group_size as usize;
        // add one to include the leading space
        1 + group_sz * group_per_panel
    }

    fn write_border(&mut self, border_elements: BorderElements) -> io::Result<()> {
        let h = border_elements.horizontal_line;
        let c = border_elements.column_separator;
        let l = border_elements.left_corner;
        let r = border_elements.right_corner;
        let h8 = h.to_string().repeat(8);
        let h_repeat = h.to_string().repeat(self.panel_sz());

        if self.show_position_panel {
            write!(self.writer, "{l}{h8}{c}")?;
        } else {
            write!(self.writer, "{l}")?;
        }

        for _ in 0..self.panels - 1 {
            write!(self.writer, "{h_repeat}{c}")?;
        }
        if self.show_char_panel {
            write!(self.writer, "{h_repeat}{c}")?;
        } else {
            write!(self.writer, "{h_repeat}")?;
        }

        if self.show_char_panel {
            for _ in 0..self.panels - 1 {
                write!(self.writer, "{h8}{c}")?;
            }
            writeln!(self.writer, "{h8}{r}")?;
        } else {
            writeln!(self.writer, "{r}")?;
        }

        Ok(())
    }

    pub fn print_header(&mut self) -> io::Result<()> {
        if let Some(e) = self.border_style.header_elems() {
            self.write_border(e)?
        }
        Ok(())
    }

    pub fn print_footer(&mut self) -> io::Result<()> {
        if let Some(e) = self.border_style.footer_elems() {
            self.write_border(e)?
        }
        Ok(())
    }

    fn print_position_panel(&mut self) -> io::Result<()> {
        self.writer.write_all(
            self.border_style
                .outer_sep()
                .encode_utf8(&mut [0; 4])
                .as_bytes(),
        )?;
        if self.show_color {
            self.writer.write_all(COLOR_OFFSET)?;
        }
        if self.show_position_panel {
            match self.squeezer {
                Squeezer::Print => {
                    self.writer.write_all(&[b'*'])?;
                    if self.show_color {
                        self.writer.write_all(COLOR_RESET)?;
                    }
                    self.writer.write_all(b"       ")?;
                }
                Squeezer::Ignore | Squeezer::Disabled | Squeezer::Delete => {
                    let byte_index: [u8; 8] = (self.idx + self.display_offset).to_be_bytes();
                    let mut i = 0;
                    while byte_index[i] == 0x0 && i < 4 {
                        i += 1;
                    }
                    for &byte in byte_index.iter().skip(i) {
                        self.writer
                            .write_all(self.byte_hex_panel_g[byte as usize].as_bytes())?;
                    }
                    if self.show_color {
                        self.writer.write_all(COLOR_RESET)?;
                    }
                }
            }
            self.writer.write_all(
                self.border_style
                    .outer_sep()
                    .encode_utf8(&mut [0; 4])
                    .as_bytes(),
            )?;
        }
        Ok(())
    }

    fn print_char(&mut self, i: u64) -> io::Result<()> {
        match self.squeezer {
            Squeezer::Print | Squeezer::Delete => self.writer.write_all(b" ")?,
            Squeezer::Ignore | Squeezer::Disabled => {
                if let Some(&b) = self.line_buf.get(i as usize) {
                    if self.show_color
                        && self.curr_color != Some(Byte(b).color(self.character_table))
                    {
                        self.writer.write_all(Byte(b).color(self.character_table))?;
                        self.curr_color = Some(Byte(b).color(self.character_table));
                    }
                    self.writer
                        .write_all(self.byte_char_panel[b as usize].as_bytes())?;
                } else {
                    self.squeezer = Squeezer::Print;
                }
            }
        }
        if i == 8 * self.panels - 1 {
            if self.show_color {
                self.writer.write_all(COLOR_RESET)?;
                self.curr_color = None;
            }
            self.writer.write_all(
                self.border_style
                    .outer_sep()
                    .encode_utf8(&mut [0; 4])
                    .as_bytes(),
            )?;
        } else if i % 8 == 7 {
            if self.show_color {
                self.writer.write_all(COLOR_RESET)?;
                self.curr_color = None;
            }
            self.writer.write_all(
                self.border_style
                    .inner_sep()
                    .encode_utf8(&mut [0; 4])
                    .as_bytes(),
            )?;
        }

        Ok(())
    }

    pub fn print_char_panel(&mut self) -> io::Result<()> {
        for i in 0..self.line_buf.len() {
            self.print_char(i as u64)?;
        }
        Ok(())
    }

    fn print_byte(&mut self, i: usize, b: u8) -> io::Result<()> {
        match self.squeezer {
            Squeezer::Print => {
                if !self.show_position_panel && i == 0 {
                    if self.show_color {
                        self.writer.write_all(COLOR_OFFSET)?;
                    }
                    self.writer
                        .write_all(self.byte_char_panel[b'*' as usize].as_bytes())?;
                    if self.show_color {
                        self.writer.write_all(COLOR_RESET)?;
                    }
                } else if i % (self.group_size as usize) == 0 {
                    self.writer.write_all(b" ")?;
                }
                for _ in 0..self.base_digits {
                    self.writer.write_all(b" ")?;
                }
            }
            Squeezer::Delete => self.writer.write_all(b"   ")?,
            Squeezer::Ignore | Squeezer::Disabled => {
                if i % (self.group_size as usize) == 0 {
                    self.writer.write_all(b" ")?;
                }
                if self.show_color && self.curr_color != Some(Byte(b).color(self.character_table)) {
                    self.writer.write_all(Byte(b).color(self.character_table))?;
                    self.curr_color = Some(Byte(b).color(self.character_table));
                }
                self.writer
                    .write_all(self.byte_hex_panel[b as usize].as_bytes())?;
            }
        }
        // byte is last in panel
        if i % 8 == 7 {
            if self.show_color {
                self.curr_color = None;
                self.writer.write_all(COLOR_RESET)?;
            }
            self.writer.write_all(b" ")?;
            // byte is last in last panel
            if i as u64 % (8 * self.panels) == 8 * self.panels - 1 {
                self.writer.write_all(
                    self.border_style
                        .outer_sep()
                        .encode_utf8(&mut [0; 4])
                        .as_bytes(),
                )?;
            } else {
                self.writer.write_all(
                    self.border_style
                        .inner_sep()
                        .encode_utf8(&mut [0; 4])
                        .as_bytes(),
                )?;
            }
        }
        Ok(())
    }

    fn reorder_buffer_to_little_endian(&self, buf: &mut Vec<u8>) {
        let n = buf.len();
        let group_sz = self.group_size as usize;

        for idx in (0..n).step_by(group_sz) {
            let remaining = n - idx;
            let total = remaining.min(group_sz);

            buf[idx..idx + total].reverse();
        }
    }

    pub fn print_bytes(&mut self) -> io::Result<()> {
        let mut buf = self.line_buf.clone();

        if matches!(self.endianness, Endianness::Little) {
            self.reorder_buffer_to_little_endian(&mut buf);
        };

        for (i, &b) in buf.iter().enumerate() {
            self.print_byte(i, b)?;
        }
        Ok(())
    }

    /// Loop through the given `Reader`, printing until the `Reader` buffer
    /// is exhausted.
    pub fn print_all<Reader: Read>(&mut self, reader: Reader) -> io::Result<()> {
        let mut is_empty = true;

        let mut buf = BufReader::new(reader);

        let leftover = loop {
            // read a maximum of 8 * self.panels bytes from the reader
            if let Ok(n) = buf.read(&mut self.line_buf) {
                if n > 0 && n < 8 * self.panels as usize {
                    // if less are read, that indicates end of file after
                    if is_empty {
                        self.print_header()?;
                        is_empty = false;
                    }
                    let mut leftover = n;
                    // loop until input is ceased
                    if let Some(s) = loop {
                        if let Ok(n) = buf.read(&mut self.line_buf[leftover..]) {
                            leftover += n;
                            // there is no more input being read
                            if n == 0 {
                                self.line_buf.resize(leftover, 0);
                                break Some(leftover);
                            }
                            // amount read has exceeded line buffer
                            if leftover >= 8 * self.panels as usize {
                                break None;
                            }
                        }
                    } {
                        break Some(s);
                    };
                } else if n == 0 {
                    // if no bytes are read, that indicates end of file
                    if self.squeezer == Squeezer::Delete {
                        // empty the last line when ending is squeezed
                        self.line_buf.clear();
                        break Some(0);
                    }
                    break None;
                }
            }
            if is_empty {
                self.print_header()?;
                is_empty = false;
            }

            // squeeze is active, check if the line is the same
            // skip print if still squeezed, otherwise print and deactivate squeeze
            if matches!(self.squeezer, Squeezer::Print | Squeezer::Delete) {
                if self
                    .line_buf
                    .chunks_exact(std::mem::size_of::<usize>())
                    .all(|w| usize::from_ne_bytes(w.try_into().unwrap()) == self.squeeze_byte)
                {
                    if self.squeezer == Squeezer::Delete {
                        self.idx += 8 * self.panels;
                        continue;
                    }
                } else {
                    self.squeezer = Squeezer::Ignore;
                }
            }

            // print the line
            self.print_position_panel()?;
            self.print_bytes()?;
            if self.show_char_panel {
                self.print_char_panel()?;
            }
            self.writer.write_all(b"\n")?;

            // increment index to next line
            self.idx += 8 * self.panels;

            // change from print to delete if squeeze is still active
            if self.squeezer == Squeezer::Print {
                self.squeezer = Squeezer::Delete;
            }

            // repeat the first byte in the line until it's a usize
            // compare that usize with each usize chunk in the line
            // if they are all the same, change squeezer to print
            let repeat_byte = (self.line_buf[0] as usize) * (usize::MAX / 255);
            if !matches!(self.squeezer, Squeezer::Disabled | Squeezer::Delete)
                && self
                    .line_buf
                    .chunks_exact(std::mem::size_of::<usize>())
                    .all(|w| usize::from_ne_bytes(w.try_into().unwrap()) == repeat_byte)
            {
                self.squeezer = Squeezer::Print;
                self.squeeze_byte = repeat_byte;
            };
        };

        // special ending

        if is_empty {
            self.base_digits = 2;
            self.print_header()?;
            if self.show_position_panel {
                write!(self.writer, "{0:9}", "│")?;
            }
            write!(
                self.writer,
                "{0:2}{1:2$}{0}{0:>3$}",
                "│",
                "No content",
                self.panel_sz() - 1,
                self.panel_sz() + 1,
            )?;
            if self.show_char_panel {
                write!(self.writer, "{0:>9}{0:>9}", "│")?;
            }
            writeln!(self.writer)?;
        } else if let Some(n) = leftover {
            // last line is incomplete
            self.print_position_panel()?;
            self.squeezer = Squeezer::Ignore;
            self.print_bytes()?;
            self.squeezer = Squeezer::Print;
            for i in n..8 * self.panels as usize {
                self.print_byte(i, 0)?;
            }
            if self.show_char_panel {
                self.squeezer = Squeezer::Ignore;
                self.print_char_panel()?;
                self.squeezer = Squeezer::Print;
                for i in n..8 * self.panels as usize {
                    self.print_char(i as u64)?;
                }
            }
            self.writer.write_all(b"\n")?;
        }

        self.print_footer()?;

        self.writer.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::str;

    use super::*;

    fn assert_print_all_output<Reader: Read>(input: Reader, expected_string: String) {
        let mut output = vec![];
        let mut printer = Printer::new(
            &mut output,
            false,
            true,
            true,
            BorderStyle::Unicode,
            true,
            2,
            1,
            Base::Hexadecimal,
            Endianness::Big,
            CharacterTable::AsciiOnly,
        );

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string,)
    }

    #[test]
    fn empty_file_passes() {
        let input = io::empty();
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│        │ No content              │                         │        │        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();
        assert_print_all_output(input, expected_string);
    }

    #[test]
    fn short_input_passes() {
        let input = io::Cursor::new(b"spam");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 73 70 61 6d             ┊                         │spam    ┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();
        assert_print_all_output(input, expected_string);
    }

    #[test]
    fn display_offset() {
        let input = io::Cursor::new(b"spamspamspamspamspam");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│deadbeef│ 73 70 61 6d 73 70 61 6d ┊ 73 70 61 6d 73 70 61 6d │spamspam┊spamspam│
│deadbeff│ 73 70 61 6d             ┊                         │spam    ┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();

        let mut output = vec![];
        let mut printer: Printer<Vec<u8>> = Printer::new(
            &mut output,
            false,
            true,
            true,
            BorderStyle::Unicode,
            true,
            2,
            1,
            Base::Hexadecimal,
            Endianness::Big,
            CharacterTable::AsciiOnly,
        );
        printer.display_offset(0xdeadbeef);

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }

    #[test]
    fn multiple_panels() {
        let input = io::Cursor::new(b"supercalifragilisticexpialidocioussupercalifragilisticexpialidocioussupercalifragilisticexpialidocious");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬─────────────────────────┬─────────────────────────┬────────┬────────┬────────┬────────┐
│00000000│ 73 75 70 65 72 63 61 6c ┊ 69 66 72 61 67 69 6c 69 ┊ 73 74 69 63 65 78 70 69 ┊ 61 6c 69 64 6f 63 69 6f │supercal┊ifragili┊sticexpi┊alidocio│
│00000020│ 75 73 73 75 70 65 72 63 ┊ 61 6c 69 66 72 61 67 69 ┊ 6c 69 73 74 69 63 65 78 ┊ 70 69 61 6c 69 64 6f 63 │ussuperc┊alifragi┊listicex┊pialidoc│
│00000040│ 69 6f 75 73 73 75 70 65 ┊ 72 63 61 6c 69 66 72 61 ┊ 67 69 6c 69 73 74 69 63 ┊ 65 78 70 69 61 6c 69 64 │ioussupe┊rcalifra┊gilistic┊expialid│
│00000060│ 6f 63 69 6f 75 73       ┊                         ┊                         ┊                         │ocious  ┊        ┊        ┊        │
└────────┴─────────────────────────┴─────────────────────────┴─────────────────────────┴─────────────────────────┴────────┴────────┴────────┴────────┘
"
        .to_owned();

        let mut output = vec![];
        let mut printer: Printer<Vec<u8>> = Printer::new(
            &mut output,
            false,
            true,
            true,
            BorderStyle::Unicode,
            true,
            4,
            1,
            Base::Hexadecimal,
            Endianness::Big,
            CharacterTable::AsciiOnly,
        );

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }

    #[test]
    fn squeeze_works() {
        let input = io::Cursor::new(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 00 00 00 00 00 00 00 00 ┊ 00 00 00 00 00 00 00 00 │⋄⋄⋄⋄⋄⋄⋄⋄┊⋄⋄⋄⋄⋄⋄⋄⋄│
│*       │                         ┊                         │        ┊        │
│00000020│ 00                      ┊                         │⋄       ┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();
        assert_print_all_output(input, expected_string);
    }

    #[test]
    fn squeeze_nonzero() {
        let input = io::Cursor::new(b"000000000000000000000000000000000");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
│00000000│ 30 30 30 30 30 30 30 30 ┊ 30 30 30 30 30 30 30 30 │00000000┊00000000│
│*       │                         ┊                         │        ┊        │
│00000020│ 30                      ┊                         │0       ┊        │
└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
"
        .to_owned();
        assert_print_all_output(input, expected_string);
    }

    #[test]
    fn squeeze_multiple_panels() {
        let input = io::Cursor::new(b"0000000000000000000000000000000000000000000000000");
        let expected_string = "\
┌────────┬─────────────────────────┬─────────────────────────┬─────────────────────────┬────────┬────────┬────────┐
│00000000│ 30 30 30 30 30 30 30 30 ┊ 30 30 30 30 30 30 30 30 ┊ 30 30 30 30 30 30 30 30 │00000000┊00000000┊00000000│
│*       │                         ┊                         ┊                         │        ┊        ┊        │
│00000030│ 30                      ┊                         ┊                         │0       ┊        ┊        │
└────────┴─────────────────────────┴─────────────────────────┴─────────────────────────┴────────┴────────┴────────┘
"
        .to_owned();

        let mut output = vec![];
        let mut printer: Printer<Vec<u8>> = Printer::new(
            &mut output,
            false,
            true,
            true,
            BorderStyle::Unicode,
            true,
            3,
            1,
            Base::Hexadecimal,
            Endianness::Big,
            CharacterTable::AsciiOnly,
        );

        printer.print_all(input).unwrap();

        let actual_string: &str = str::from_utf8(&output).unwrap();
        assert_eq!(actual_string, expected_string)
    }
}
