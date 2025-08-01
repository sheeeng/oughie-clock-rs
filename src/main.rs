mod character;
mod cli;
mod clock;
#[macro_use]
mod color;
mod config;
mod error;
mod position;
mod segment;
mod state;

use std::process;

use crate::{color::Color, error::Error, state::State};

fn main() {
    if let Err(err) = (|| State::new()?.run().map_err(Error::Io))() {
        println!("{}error:{} {err}", esc!("1;31"), Color::RESET);
        process::exit(1);
    }
}
