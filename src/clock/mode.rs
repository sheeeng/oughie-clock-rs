use super::{counter::Counter, time_zone::TimeZone};

pub enum ClockMode {
    Counter(Counter),
    Time {
        time_zone: TimeZone,
        date_format: String,
    },
}

impl ClockMode {
    pub fn get_time(&self) -> (u32, u32, u32) {
        match self {
            Self::Counter(counter) => counter.get_time(),
            Self::Time { time_zone, .. } => time_zone.get_time(),
        }
    }

    pub fn text(&self) -> String {
        match self {
            Self::Counter(counter) => counter.text.to_string(),
            Self::Time {
                time_zone,
                date_format,
            } => time_zone.text(date_format),
        }
    }
}

impl Default for ClockMode {
    fn default() -> Self {
        Self::Time {
            time_zone: TimeZone::Local,
            date_format: "%d-%m-%Y".to_string(),
        }
    }
}
