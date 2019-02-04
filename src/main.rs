#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate atty;
extern crate ctrlc;

use std::fs::File;
use std::io::{self, prelude::*, StdoutLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::{App, AppSettings, Arg};

use ansi_term::Color;
use ansi_term::Color::Fixed;

use atty::Stream;

const BUFFER_SIZE: usize = 256;

const COLOR_NULL: Color = Fixed(242); // grey
const COLOR_OFFSET: Color = Fixed(242); // grey
const COLOR_ASCII_PRINTABLE: Color = Color::Cyan;
const COLOR_ASCII_WHITESPACE: Color = Color::Green;
const COLOR_ASCII_OTHER: Color = Color::Purple;
const COLOR_NONASCII: Color = Color::Yellow;

enum ByteCategory {
    Null,
    AsciiPrintable,
    AsciiWhitespace,
    AsciiOther,
    NonAscii,
}

#[derive(Copy, Clone)]
struct Byte(u8);

impl Byte {
    fn category(self) -> ByteCategory {
        if self.0 == 0x00 {
            ByteCategory::Null
        } else if self.0.is_ascii_alphanumeric()
            || self.0.is_ascii_punctuation()
            || self.0.is_ascii_graphic()
        {
            ByteCategory::AsciiPrintable
        } else if self.0.is_ascii_whitespace() {
            ByteCategory::AsciiWhitespace
        } else if self.0.is_ascii() {
            ByteCategory::AsciiOther
        } else {
            ByteCategory::NonAscii
        }
    }

    fn color(self) -> &'static Color {
        use ByteCategory::*;

        match self.category() {
            Null => &COLOR_NULL,
            AsciiPrintable => &COLOR_ASCII_PRINTABLE,
            AsciiWhitespace => &COLOR_ASCII_WHITESPACE,
            AsciiOther => &COLOR_ASCII_OTHER,
            NonAscii => &COLOR_NONASCII,
        }
    }

    fn as_char(self) -> char {
        use ByteCategory::*;

        match self.category() {
            Null => '0',
            AsciiPrintable => self.0 as char,
            AsciiWhitespace if self.0 == 0x20 => ' ',
            AsciiWhitespace => '_',
            AsciiOther => '•',
            NonAscii => '×',
        }
    }
}

struct Printer<'a> {
    idx: usize,
    /// The raw bytes used as input for the current line.
    raw_line: Vec<u8>,
    /// The buffered line built with each byte, ready to print to stdout.
    buffer_line: Vec<u8>,
    stdout: StdoutLock<'a>,
    show_color: bool,
    byte_hex_table: Vec<String>,
    byte_char_table: Vec<String>,
}

impl<'a> Printer<'a> {
    fn new(stdout: StdoutLock, show_color: bool) -> Printer {
        Printer {
            idx: 1,
            raw_line: vec![],
            buffer_line: vec![],
            stdout,
            show_color,
            byte_hex_table: (0u8..=u8::max_value())
                .map(|i| {
                    let byte_hex = format!("{:02x} ", i);
                    if show_color {
                        Byte(i).color().paint(byte_hex).to_string()
                    } else {
                        byte_hex
                    }
                })
                .collect(),
            byte_char_table: (0u8..=u8::max_value())
                .map(|i| {
                    let byte_char = format!("{}", Byte(i).as_char());
                    if show_color {
                        Byte(i).color().paint(byte_char).to_string()
                    } else {
                        byte_char
                    }
                })
                .collect(),
        }
    }

    fn header(&mut self) {
        writeln!(
            self.stdout,
            "┌{0:─<8}┬{0:─<25}┬{0:─<25}┬{0:─<8}┬{0:─<8}┐",
            ""
        )
        .ok();
    }

    fn footer(&mut self) {
        writeln!(
            self.stdout,
            "└{0:─<8}┴{0:─<25}┴{0:─<25}┴{0:─<8}┴{0:─<8}┘",
            ""
        )
        .ok();
    }

    fn print_byte(&mut self, b: u8) -> io::Result<()> {
        if self.idx % 16 == 1 {
            let style = COLOR_OFFSET.normal();
            let byte_index = format!("{:08x}", self.idx - 1);
            let formatted_string = if self.show_color {
                format!("{}", style.paint(byte_index))
            } else {
                byte_index
            };
            let _ = write!(&mut self.buffer_line, "│{}│ ", formatted_string);
        }

        write!(&mut self.buffer_line, "{}", self.byte_hex_table[b as usize])?;
        self.raw_line.push(b);

        match self.idx % 16 {
            8 => {
                let _ = write!(&mut self.buffer_line, "┊ ");
            }
            0 => self.print_textline()?,
            _ => {}
        }

        self.idx += 1;

        Ok(())
    }

    fn print_textline(&mut self) -> io::Result<()> {
        let len = self.raw_line.len();

        if len == 0 {
            return Ok(());
        }

        if len < 8 {
            let _ = write!(
                &mut self.buffer_line,
                "{0:1$}┊{0:2$}│",
                "",
                3 * (8 - len),
                1 + 3 * 8
            );
        } else {
            let _ = write!(&mut self.buffer_line, "{0:1$}│", "", 3 * (16 - len));
        }

        let mut idx = 1;
        for &b in self.raw_line.iter() {
            let _ = write!(
                &mut self.buffer_line,
                "{}",
                self.byte_char_table[b as usize]
            );

            if idx == 8 {
                let _ = write!(&mut self.buffer_line, "┊");
            }

            idx += 1;
        }

        if len < 8 {
            let _ = writeln!(&mut self.buffer_line, "{0:1$}┊{0:2$}│ ", "", 8 - len, 8);
        } else {
            let _ = writeln!(&mut self.buffer_line, "{0:1$}│", "", 16 - len);
        }
        self.stdout.write_all(&self.buffer_line)?;

        self.raw_line.clear();
        self.buffer_line.clear();

        Ok(())
    }
}

fn run() -> Result<(), Box<::std::error::Error>> {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file").help("File to display"))
        .arg(
            Arg::with_name("length")
                .short("n")
                .long("length")
                .takes_value(true)
                .value_name("N")
                .help("Read only N bytes from the input"),
        )
        .arg(
            Arg::with_name("c")
                .short("c")
                .long("c")
                .takes_value(true)
                .hidden(true),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .takes_value(true)
                .value_name("when")
                .possible_values(&["always", "auto", "never"])
                .default_value("always")
                .help(
                    "When to use colors. The auto-mode only displays colors if the output \
                     goes to an interactive terminal",
                ),
        );

    let matches = app.get_matches_safe()?;

    let stdin = io::stdin();

    let mut reader: Box<dyn Read> = match matches.value_of("file") {
        Some(filename) => Box::new(File::open(filename)?),
        None => Box::new(stdin.lock()),
    };

    if let Some(length) = matches
        .value_of("length")
        .and_then(|n| n.parse::<u64>().ok())
    {
        reader = Box::new(reader.take(length));
    }

    if let Some(length) = matches.value_of("c").and_then(|n| n.parse::<u64>().ok()) {
        reader = Box::new(reader.take(length));
    }

    let show_color = match matches.value_of("color") {
        Some("never") => false,
        Some("auto") => atty::is(Stream::Stdout),
        _ => true,
    };

    // Set up Ctrl-C handler
    let cancelled = Arc::new(AtomicBool::new(false));
    let c = cancelled.clone();

    ctrlc::set_handler(move || {
        c.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let stdout = io::stdout();
    let mut printer = Printer::new(stdout.lock(), show_color);
    printer.header();

    let mut buffer = [0; BUFFER_SIZE];
    'mainloop: loop {
        let size = reader.read(&mut buffer)?;
        if size == 0 {
            break;
        }

        if cancelled.load(Ordering::SeqCst) {
            eprintln!("hexyl has been cancelled.");
            std::process::exit(130); // Set exit code to 128 + SIGINT
        }

        for b in &buffer[..size] {
            let res = printer.print_byte(*b);

            if res.is_err() {
                // Broken pipe
                break 'mainloop;
            }
        }
    }

    // Finish last line
    printer.print_textline().ok();
    printer.footer();

    Ok(())
}

fn main() {
    // Enable ANSI support for Windows
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let result = run();

    if let Err(err) = result {
        if let Some(clap_err) = err.downcast_ref::<clap::Error>() {
            eprint!("{}", clap_err); // Clap errors already have newlines

            match clap_err.kind {
                // The exit code should not indicate an error for --help / --version
                clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
                    std::process::exit(0)
                }
                _ => (),
            }
        } else {
            eprintln!("Error: {}", err);
        }
        std::process::exit(1);
    }
}
