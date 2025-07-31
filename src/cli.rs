use crate::{color::Color, config::Config, position::Position};

use clap::{
    builder::styling::{AnsiColor, Styles},
    Parser, Subcommand,
};
use serde::Deserialize;

#[derive(Parser)]
#[clap(version = "v0.1.215, (C) 2024 Oughie", hide_possible_values = true, styles = Self::STYLES)]
pub struct Args {
    #[clap(subcommand)]
    pub mode: Option<Mode>,
    #[doc = "Specify the clock color"]
    #[clap(long, short)]
    pub color: Option<Color>,
    #[doc = "Set the position along the horizontal axis"]
    #[clap(long, short)]
    pub x_pos: Option<Position>,
    #[doc = "Set the position along the vertical axis"]
    #[clap(long, short)]
    pub y_pos: Option<Position>,
    #[doc = "Set the date format"]
    #[clap(long)]
    pub fmt: Option<String>,
    #[doc = "Use the 12h format"]
    #[clap(short = 't')]
    pub use_12h: bool,
    #[doc = "Set the polling interval in milliseconds"]
    #[clap(long, short)]
    pub interval: Option<u64>,
    #[doc = "Use UTC time"]
    #[clap(long)]
    pub utc: bool,
    #[doc = "Do not show seconds"]
    #[clap(long, short = 's')]
    pub hide_seconds: bool,
    #[doc = "Set the colon to blink"]
    #[clap(long, short = 'B')]
    pub blink: bool,
    #[doc = "Use bold text"]
    #[clap(long, short)]
    pub bold: bool,
}

impl Args {
    const STYLES: Styles = Styles::styled()
        .header(AnsiColor::Green.on_default().bold().underline())
        .usage(AnsiColor::Green.on_default().bold().underline())
        .literal(AnsiColor::Blue.on_default().bold())
        .placeholder(AnsiColor::Yellow.on_default().italic());
}

#[derive(Clone, Subcommand, Deserialize)]
pub enum Mode {
    #[doc = "Display the current time (default)"]
    Clock,
    #[doc = "Create a timer"]
    Timer(TimerArgs),
    #[doc = "Start a stopwatch"]
    Stopwatch,
}

#[derive(clap::Args, Clone, Deserialize)]
pub struct TimerArgs {
    #[doc = "Add seconds to the timer"]
    #[clap(long, short = 'S')]
    pub seconds: Option<u64>,
    #[doc = "Add minutes to the timer"]
    #[clap(long, short = 'M')]
    pub minutes: Option<u64>,
    #[doc = "Add hours to the timer"]
    #[clap(long, short = 'H')]
    pub hours: Option<u64>,
    #[doc = "Terminate the application when the timer finishes"]
    #[clap(long, short)]
    pub kill: bool,
}

impl Args {
    pub fn overwrite(self, config: &mut Config) {
        if let Some(color) = self.color {
            config.general.color = color;
        }
        if let Some(interval) = self.interval {
            config.general.interval = interval;
        }
        if let Some(x_pos) = self.x_pos {
            config.position.x = x_pos;
        }
        if let Some(y_pos) = self.y_pos {
            config.position.y = y_pos;
        }
        if self.blink {
            config.general.blink = true;
        }
        if self.bold {
            config.general.bold = true;
        }
        if let Some(fmt) = self.fmt {
            config.date.fmt = fmt;
        }
        if self.use_12h {
            config.date.use_12h = true;
        }
        if self.utc {
            config.date.utc = true;
        }
        if self.hide_seconds {
            config.date.hide_seconds = true;
        }
    }
}
