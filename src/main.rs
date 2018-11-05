#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{self, prelude::*};

use clap::{App, AppSettings, Arg};

const BUFFER_SIZE: usize = 64;

struct Printer {
    idx: usize,
}

impl Printer {
    fn print_byte(&mut self, b: u8) {
        print!("{:02x} ", b);

        match self.idx % 16 {
            8 => print!(" "),
            0 => println!(),
            _ => {}
        }

        self.idx += 1;
    }
}

fn run() -> io::Result<()> {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .arg(Arg::with_name("file").help("to do").required(true));

    let matches = app.get_matches();

    let filename = matches.value_of("file").unwrap();

    let mut buffer = [0; BUFFER_SIZE];
    let mut file = File::open(filename)?;

    let mut printer = Printer { idx: 1 };
    loop {
        let size = file.read(&mut buffer)?;
        if size == 0 {
            break;
        }

        for b in &buffer[..size] {
            printer.print_byte(*b);
        }
    }
    println!();

    Ok(())
}

fn main() {
    let result = run();
    match result {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
        Ok(()) => {}
    }
}
