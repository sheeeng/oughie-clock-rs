use std::{fmt, io};

use crate::{clock::counter::Counter, color::Color};

pub enum Error {
    TimerDurationTooLong(u64),
    PathIsNonUnicode(String),
    FailedToReadFile(String, String),
    InvalidToml(String, String),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimerDurationTooLong(duration) => write!(
                f,
                "The timer duration must be shorter than {} hours:{} {duration}s >= {}s",
                Counter::MAX_TIMER_HOURS,
                Color::RESET,
                Counter::MAX_TIMER_SECONDS
            ),
            Self::PathIsNonUnicode(path) => write!(
                f,
                "Configuration path is invalid unicode:{} `{path}`",
                Color::RESET
            ),
            Self::FailedToReadFile(path, err) => {
                write!(f, "Failed to read file `{path}`:{}\n{err}", Color::RESET)
            }
            Self::InvalidToml(path, err) => write!(
                f,
                "Failed to parse configuration file `{path}`:{}\n{err}",
                Color::RESET
            ),
            Self::Io(err) => write!(f, "IO Error:{}\n{err}", Color::RESET),
        }
    }
}
