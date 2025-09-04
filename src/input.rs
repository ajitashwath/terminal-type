use crossterm::event::{KeyCode, KeyEvent};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::{test::Test, stats::LiveStats};

#[derive(Debug, Clone)]
pub struct KeystrokeData {
    pub character: char,
    pub timestamp: Instant,
    pub is_correct: bool,
    pub is_correction: bool,
}

pub struct InputHandler {
    typed_text: String,
    keystrokes: Vec<KeystrokeData>,
    start_time: Option<Instant>,
    last_keystroke_time: Option<Instant>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            typed_text: String::new(),
            keystrokes: Vec::new(),
            start_time: None,
            last_keystroke_time: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent, test: &Test) -> Result<(), Box<dyn std::error::Error>> {
        let now = Instant::now();
        if self.start_time.is_none() {
            self.start_time = Some(now);
            test.start();
        }

        match key.code {
            KeyCode::Char(ch) => {
                self.handle_character(ch, now, test);
            }
            KeyCode::Backspace => {
                self.handle_backspace(now);
            }
            KeyCode::Space => {
                self.handle_character(' ', now, test);
            }
            _ => {}
        }
        self.last_keystroke_time = Some(now);
        Ok(())
    }
    
}