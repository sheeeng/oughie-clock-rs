#[allow(dead_code)]
#[path = "src/color.rs"]
mod color;

#[allow(dead_code)]
#[path = "src/position.rs"]
mod position;

#[path = "src/cli/args.rs"]
mod args;

use std::{fs, io, path::PathBuf};

use clap::{Command, CommandFactory, ValueEnum};
use clap_complete::Shell;

use crate::args::Args;

fn generate_shell_completions(mut cmd: Command) -> io::Result<()> {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/completions");

    fs::create_dir_all(&out_dir)?;

    for shell in Shell::value_variants() {
        clap_complete::generate_to(*shell, &mut cmd, "clock-rs", &out_dir)?;
    }

    Ok(())
}

fn main() {
    let cmd = Args::command();

    if let Err(err) = generate_shell_completions(cmd) {
        println!("cargo::warning=error generating completions for clock-rs: {err}");
    }
}
