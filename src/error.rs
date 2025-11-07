use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("the seconds part of the timer must be under 60 seconds: {0}s >= 60s")]
    TooManySeconds(u64),
    #[error("the minutes part of the timer must be under 60 minutes: {0}m >= 60m")]
    TooManyMinutes(u64),
    #[error("the hours part of the timer must be under 100 hours: {0}h >= 100h")]
    TooManyHours(u64),
    #[error("the formatted date exceeds the clock's width: {fmt_len} > {max_len}")]
    DateFormatTooWide { fmt_len: u16, max_len: u16 },
    #[error("failed to format the date string `{fmt}`: {err}")]
    DateFormatInvalid { fmt: String, err: String },
    #[error("configuration path is invalid unicode: `{0}`")]
    NonUnicodePath(String),
    #[error("failed to read file `{path}`: {err}")]
    ReadFile { path: String, err: String },
    #[error("failed to parse configuration file `{path}`:\n{err}")]
    ParseToml { path: String, err: String },
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
