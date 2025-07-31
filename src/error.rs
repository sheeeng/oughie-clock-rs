use std::{fmt, io};

use crate::color::Color;

pub enum Error {
    TooManySeconds(u64),
    TooManyMinutes(u64),
    TooManyHours(u64),
    NonUnicodePath(String),
    ReadFile { path: String, err: String },
    ParseToml { path: String, err: String },
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManySeconds(seconds) => write!(
                f,
                "The seconds part of the timer must be under 60 seconds:{} {seconds}s >= 60s",
                Color::RESET,
            ),
            Self::TooManyMinutes(minutes) => write!(
                f,
                "The minutes part of the timer must be under 60 minutes:{} {minutes}m >= 60m",
                Color::RESET,
            ),
            Self::TooManyHours(hours) => write!(
                f,
                "The hours part of the timer must be under 100 hours:{} {hours}h >= 100h",
                Color::RESET,
            ),
            Self::NonUnicodePath(path) => write!(
                f,
                "Configuration path is invalid unicode:{} `{path}`",
                Color::RESET
            ),
            Self::ReadFile { path, err } => {
                write!(f, "Failed to read file `{path}`:{}\n{err}", Color::RESET)
            }
            Self::ParseToml { path, err } => write!(
                f,
                "Failed to parse configuration file `{path}`:{}\n{err}",
                Color::RESET
            ),
            Self::Io(err) => write!(f, "IO Error:{}\n{err}", Color::RESET),
        }
    }
}
