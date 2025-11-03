use std::{
    env::{self, VarError},
    fs,
    path::Path,
};

use serde::Deserialize;

use crate::{color::Color, error::Error, position::Position};

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub position: PositionConfig,
    pub date: DateConfig,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub color: Color,
    pub interval: u64,
    pub blink: bool,
    pub bold: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            interval: 200,
            color: Color::default(),
            blink: false,
            bold: false,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct PositionConfig {
    #[serde(rename = "horizontal")]
    pub x: Position,
    #[serde(rename = "vertical")]
    pub y: Position,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct DateConfig {
    pub fmt: String,
    pub use_12h: bool,
    pub utc: bool,
    pub hide_seconds: bool,
}

impl Default for DateConfig {
    fn default() -> Self {
        Self {
            fmt: "%d-%m-%Y".to_string(),
            use_12h: false,
            utc: false,
            hide_seconds: false,
        }
    }
}

impl Config {
    pub fn parse() -> Result<Self, Error> {
        let path = match env::var("CONF_PATH") {
            Ok(path) => match path.as_str() {
                "None" => None,
                _ => Some(path),
            },
            Err(VarError::NotUnicode(path)) => {
                return Err(Error::NonUnicodePath(path.to_string_lossy().to_string()));
            }
            Err(VarError::NotPresent) => match dirs::config_local_dir() {
                Some(config_local_dir) => {
                    match config_local_dir.join("clock-rs").join("conf.toml").to_str() {
                        Some(path) => {
                            if Path::new(path).exists() {
                                Some(path.to_string())
                            } else {
                                None
                            }
                        }
                        None => {
                            return Err(Error::NonUnicodePath(
                                config_local_dir.to_string_lossy().to_string(),
                            ));
                        }
                    }
                }
                None => None,
            },
        };

        let Some(file_path) = path else {
            return Ok(Config::default());
        };

        let config_str = fs::read_to_string(&file_path).map_err(|err| Error::ReadFile {
            path: file_path.clone(),
            err: err.to_string(),
        })?;

        toml::from_str(&config_str).map_err(|err| Error::ParseToml {
            path: file_path,
            err: err.to_string(),
        })
    }
}
