
use clock_core::timer::{Timer, TimerData};
use chrono::Duration;
use hhmmss::Hhmmss;

#[derive(Copy, Clone)]
enum TimerViewState {
    Config,
    Running,
    Finished,
}

struct TimerViewConfig {
    h: u8,
    m: u8,
    s: u8,
    focus: u8,
    input_buffer: Vec<u8>,
}

pub struct TimerView {
    timer: Timer,
    remaining: Duration,
    state: TimerViewState,
    config: TimerViewConfig,
}

impl TimerView {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(Duration::zero()),
            remaining: Duration::zero(),
            state: TimerViewState::Config,
            config: TimerViewConfig {
                h: 0,
                m: 0,
                s: 0,
                focus: 0,
                input_buffer: Vec::new(),
            },
        }
    }

    pub fn start(&mut self) {
        let seconds = self.config.h as i64 * 3600
            + self.config.m as i64 * 60
            + self.config.s as i64;
        self.timer = Timer::new(Duration::seconds(seconds));
        self.state = TimerViewState::Running;
        self.timer.pause_or_resume()
    }

    pub fn update(&mut self) {
        match self.state {
            TimerViewState::Config => {
                if self.config.input_buffer.len() > 0 {
                    let input = self.config.input_buffer.pop().unwrap();
                    match self.config.focus {
                        0 => self.config.h = input,
                        1 => self.config.m = input,
                        2 => self.config.s = input,
                        _ => {}
                    }
                }
            }
            TimerViewState::Running => {
                if self.remaining.num_seconds() == 0 {
                    self.state = TimerViewState::Finished;
                }
            }
            TimerViewState::Finished => {}
        }
    }

    pub fn render(&self) -> String {
        match self.state {
            TimerViewState::Config => {
                format!("{}:{}:{}", self.config.h, self.config.m, self.config.s)
            }
            TimerViewState::Running => {
                format!("{}", self.remaining.hhmmssxxx())
            }
            TimerViewState::Finished => {
                format!("Finished!")
            }
        }
    }
}
