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

    pub fn handle_key(&mut self, key: KeyEvent, test: &mut Test) -> Result<(), Box<dyn std::error::Error>> {
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
            _ => {}
        }
        self.last_keystroke_time = Some(now);
        Ok(())
    }

    fn handle_character(&mut self, ch: char, timestamp: Instant, test: &Test) {
        let target_text = test.get_text();
        let current_pos = self.typed_text.len();
        if current_pos >= target_text.len() { return; }
        
        let expected_char = target_text.chars().nth(current_pos);
        let is_correct = expected_char == Some(ch);
        
        self.typed_text.push(ch);
        self.keystrokes.push(KeystrokeData { 
            character: ch, 
            timestamp, 
            is_correct, 
            is_correction: false,
        });
    }

    fn handle_backspace(&mut self, timestamp: Instant) {
        if !self.typed_text.is_empty() {
            let removed_char = self.typed_text.pop().unwrap();
            self.keystrokes.push(KeystrokeData { 
                character: removed_char, 
                timestamp, 
                is_correct: true,
                is_correction: true 
            });
        }
    }

    pub fn get_typed_text(&self) -> &str {
        &self.typed_text
    }
    
    pub fn get_keystrokes(&self) -> &[KeystrokeData] {
        &self.keystrokes
    }

    pub fn get_start_time(&self) -> Option<Instant> {
        self.start_time
    }

    pub fn get_progress(&self, target_text: &str) -> f64 {
        if target_text.is_empty() {
            return 1.0;
        }
        self.typed_text.len() as f64 / target_text.len() as f64
    }

    pub fn get_typed_words(&self) -> usize {
        if self.typed_text.is_empty() {
            0
        } else {
            self.typed_text.split_whitespace().count()
        }
    }

    pub fn get_live_stats(&self, elapsed: Duration) -> LiveStats {
        let elapsed_minutes = elapsed.as_secs_f64() / 60.0;
        if elapsed_minutes == 0.0 {
            return LiveStats {
                wpm: 0.0,
                accuracy: 1.0,
                error_count: 0,
            };
        }
        
        let typing_keystrokes: Vec<_> = self.keystrokes.iter().filter(|k| !k.is_correction).collect();
        let total_chars = typing_keystrokes.len();
        let correct_chars = typing_keystrokes.iter().filter(|k| k.is_correct).count();
        let error_count = total_chars - correct_chars;
        
        let words_typed = total_chars as f64 / 5.0;
        let wpm = words_typed / elapsed_minutes;
        
        let accuracy = if total_chars > 0 {
            correct_chars as f64 / total_chars as f64
        } else {
            1.0
        };
        
        LiveStats {
            wpm,
            accuracy,
            error_count,
        }
    }

    pub fn calculate_error_frequency(&self, target_text: &str) -> HashMap<char, usize> {
        let mut frequency = HashMap::new();
        let target_chars: Vec<char> = target_text.chars().collect();
        let mut pos = 0;
        
        for keystroke in &self.keystrokes {
            if keystroke.is_correction {
                if pos > 0 {
                    pos -= 1;
                }
            } else {
                if pos < target_chars.len() {
                    let expected = target_chars[pos];
                    if keystroke.character != expected {
                        *frequency.entry(expected).or_insert(0) += 1;
                    }
                }
                pos += 1;
            }
        }
        frequency
    }

    pub fn get_speed_over_time(&self) -> Vec<(f64, f64)> {
        if self.start_time.is_none() || self.keystrokes.is_empty() {
            return vec![(0.0, 0.0)];
        }
        
        let start = self.start_time.unwrap();
        let mut speed_points = vec![(0.0, 0.0)];
        let mut char_count = 0;

        for keystroke in &self.keystrokes {
            if !keystroke.is_correction {
                char_count += 1;
                let elapsed_seconds = keystroke.timestamp.duration_since(start).as_secs_f64();
                let elapsed_minutes = elapsed_seconds / 60.0;
                if elapsed_minutes > 0.0 {
                    let words = char_count as f64 / 5.0;
                    let wpm = words / elapsed_minutes;
                    speed_points.push((elapsed_minutes, wpm));
                }
            }
        }
        speed_points
    }

    pub fn get_consistency_score(&self) -> f64 {
        if self.keystrokes.len() < 2 {
            return 1.0;
        }
        
        let speed_points = self.get_speed_over_time();
        if speed_points.len() < 3 {
            return 1.0;
        }
        
        let wpms: Vec<f64> = speed_points.iter().map(|(_, wpm)| *wpm).collect();
        let mean_wpm = wpms.iter().sum::<f64>() / wpms.len() as f64;
        let variance = wpms.iter().map(|wpm| (wpm - mean_wpm).powi(2)).sum::<f64>() / wpms.len() as f64;
        let std_dev = variance.sqrt();
        
        if mean_wpm > 0.0 {
            (1.0 - (std_dev / mean_wpm)).max(0.0)
        } else {
            1.0
        }
    }
}