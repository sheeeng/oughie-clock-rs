use std::{
    io::{self, Write},
    time::Duration,
};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

#[cfg(unix)]
use signal_hook::{consts, flag};

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

        let clock_mode = Self::clock_mode(mode, &config)?;
        let interval = Duration::from_millis(config.general.interval);
        let (width, height) = terminal::size().map_err(Error::Io)?;
        let mut clock = Clock::new(config, clock_mode);

        clock.update_padding(width, height)?;

        Ok(Self { clock, interval })
    }

    fn clock_mode(mode: Option<Mode>, config: &Config) -> Result<ClockMode, Error> {
        Ok(match mode {
            Some(Mode::Clock) | None => ClockMode::Time {
                time_zone: TimeZone::from_utc(config.date.utc),
                date_format: config.date.fmt.clone(),
            },
            Some(Mode::Timer(args)) => {
                let total_seconds =
                    if args.seconds.is_none() && args.minutes.is_none() && args.hours.is_none() {
                        Counter::DEFAULT_TIMER_DURATION
                    } else {
                        let seconds = args.seconds.unwrap_or_default();
                        let minutes = args.minutes.unwrap_or_default();
                        let hours = args.hours.unwrap_or_default();

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
        })
    }

    pub fn run(mut self) -> Result<(), Error> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, Hide)?;

        let reload_config = Arc::new(AtomicBool::new(false));

        #[cfg(unix)]
        flag::register(consts::SIGUSR1, Arc::clone(&reload_config))?;

        loop {
            if reload_config.swap(false, Ordering::Relaxed) {
                let clock = &mut self.clock;
                let config = Config::parse()?;

                clock.color = config.general.color;
                self.interval = Duration::from_millis(config.general.interval);
                clock.blink = config.general.blink;
                clock.bold = config.general.bold;

                clock.x_pos = config.position.x;
                clock.y_pos = config.position.y;

                clock.use_12h = config.date.use_12h;
                clock.hide_seconds = config.date.hide_seconds;

                if let ClockMode::Time {
                    time_zone,
                    date_format,
                } = &mut self.clock.mode
                {
                    *time_zone = TimeZone::from_utc(config.date.utc);
                    *date_format = config.date.fmt;
                }

                let (width, height) = terminal::size()?;

                execute!(stdout, terminal::Clear(ClearType::All))?;
                self.clock.update_padding(width, height)?;
            }

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

                            self.clock.update_padding(width, height)?;
                            execute!(stdout, Clear(ClearType::All))?;
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Char('r'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => reload_config.store(true, Ordering::Relaxed),
                    _ => (),
                },
                Event::Resize(width, height) => {
                    self.clock.update_padding(width, height)?;
                    execute!(stdout, Clear(ClearType::All))?;
                }
                _ => (),
            }
        }
    }

    pub fn render(&self) -> Result<(), Error> {
        let mut stdout = io::stdout();
        let (width, height) = terminal::size()?;

        if self.clock.is_too_large(width, height) {
            return Ok(());
        }

        let lock = stdout.lock();
        let mut w = io::BufWriter::new(lock);

        execute!(stdout, MoveTo(0, self.clock.padding.top))?;
        self.clock.fmt(&mut w)?;

        w.flush()?;

        Ok(())
    }

    pub fn exit() {
        execute!(io::stdout(), LeaveAlternateScreen, Show).expect(
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
