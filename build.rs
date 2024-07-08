use clap_complete::{generate_to, shells::Bash, shells::Zsh};
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => {
            println!("cargo:warning=Will not generate any shell completion files. OUT_DIR environment variable is not set, which shouldn't happen in normal conditions.");
            return Ok(());
        }
        Some(outdir) => outdir,
    };

    let mut cmd = build_cli();

    let path = match env!("SHELL") {
        "/usr/bin/zsh" => generate_to(Zsh, &mut cmd, "umivarcalextract", outdir)?,
        "/usr/bin/bash" => generate_to(Bash, &mut cmd, "umivarcalextract", outdir)?,
        _ => generate_to(Zsh, &mut cmd, "umivarcalextract", outdir)?,
    };

    println!("cargo:warning=completion file is generated: {path:?}");

    return Ok(())
}
