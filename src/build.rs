#[macro_use]
extern crate clap;

use clap::ValueEnum;
use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;

include!("cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = build_cli();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "hexyl", &outdir)?;
    }

    Ok(())
}
