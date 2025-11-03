use std::{
    io::{self, Write},
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{
    cli::{Args, Mode},
    clock::{counter::Counter, counter::CounterType, mode::ClockMode, time_zone::TimeZone, Clock},
    config::Config,
    error::Error,
};

pub struct State {
    clock: Clock,
    interval: Duration,
}

impl State {
    pub fn new() -> Result<Self, Error> {
        let args = Args::parse();
        let mut config = Config::parse()?;
        let mode = args.mode.clone();

        args.overwrite(&mut config);

        let clock_mode = match mode {
            Some(Mode::Clock) | None => ClockMode::Time {
                time_zone: if config.date.utc {
                    TimeZone::Utc
                } else {
                    TimeZone::Local
                },
                date_format: config.date.fmt.clone(),
            },
            Some(Mode::Timer(args)) => {
                let total_seconds =
                    if args.seconds.is_none() && args.minutes.is_none() && args.hours.is_none() {
                        Counter::DEFAULT_TIMER_DURATION
                    } else {
                        let seconds = args.seconds.unwrap_or(0);
                        let minutes = args.minutes.unwrap_or(0);
                        let hours = args.hours.unwrap_or(0);

                        if seconds >= 60 {
                            return Err(Error::TooManySeconds(seconds));
                        }

                        if minutes >= 60 {
                            return Err(Error::TooManyMinutes(minutes));
                        }

                        if hours >= 100 {
                            return Err(Error::TooManyHours(hours));
                        }

                        seconds + 60 * minutes + 3600 * hours
                    };

                ClockMode::Counter(Counter::new(CounterType::Timer {
                    duration: Duration::from_secs(total_seconds),
                    kill: args.kill,
                }))
            }
            Some(Mode::Stopwatch) => ClockMode::Counter(Counter::new(CounterType::Stopwatch)),
        };

        let interval = config.general.interval;

        let (width, height) = terminal::size().map_err(Error::Io)?;
        let mut clock = Clock::new(config, clock_mode).map_err(Error::Io)?;

        clock.update_position(width, height);

        Ok(Self {
            clock,
            interval: Duration::from_millis(interval),
        })
    }

    pub fn run(mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;

        loop {
            self.render()?;

            if !event::poll(self.interval)? {
                continue;
            }

            match event::read()? {
                Event::Key(key_event) => match key_event {
                    KeyEvent {
                        code: KeyCode::Esc | KeyCode::Char('Q' | 'q'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    }
                    | KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => return Ok(()),
                    KeyEvent {
                        code: KeyCode::Char(character @ ('P' | 'p' | 'R' | 'r')),
                        kind: KeyEventKind::Press,
                        modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                        ..
                    } => {
                        if let ClockMode::Counter(counter) = &mut self.clock.mode {
                            if character == 'P' || character == 'p' {
                                counter.toggle_pause();
                            } else {
                                counter.restart();
                            }

                            let (width, height) = terminal::size()?;
                            self.clock.update_position(width, height);
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                    }
                    _ => (),
                },
                Event::Resize(width, height) => {
                    self.clock.update_position(width, height);
                    execute!(stdout, Clear(ClearType::All))?;
                }
                _ => (),
            }
        }
    }

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        let (width, height) = terminal::size()?;

        if self.clock.is_too_large(width.into(), height.into()) {
            return Ok(());
        }

        let lock = stdout.lock();
        let mut w = io::BufWriter::new(lock);

        execute!(stdout, MoveTo(0, self.clock.y))?;

        write!(w, "{}", self.clock)?;

        w.flush()
    }

    pub fn exit() {
        let mut stdout = io::stdout();

        execute!(stdout, LeaveAlternateScreen, Show).expect(
            "error: failed to leave alternate screen, you might have to restart your terminal",
        );
        terminal::disable_raw_mode()
            .expect("error: failed to disable raw mode, you might have to restart your terminal");
    }
}

impl Drop for State {
    fn drop(&mut self) {
        Self::exit();
    }
}
