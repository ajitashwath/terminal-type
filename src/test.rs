use std::time::{Duration, Instant};

use crate::{app::TestMode, config::Config, utils};

pub struct Test {
    text: String,
    mode: TestMode,
    start_time: Option<Instant>,
    duration_limit: Option<Duration>,
    word_limit: Option<usize>,
}

impl Test {
    pub fn new(mode: &TestMode, config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let (text, duration_limit, word_limit) = match mode {
            TestMode::Timed(seconds) => {
                let text = Self::generate_text_for_duration(*seconds, config)?;
                (text, Some(Duration::from_secs(*seconds as u64)), None)
            }
            TestMode::WordCount(count) => {
                let text = utils::generate_random_words(*count);
                (text, None, Some(*count))
            }
            TestMode::Text(custom_text) => {
                (custom_text.clone(), None, None)
            }
        };

        Ok(Self {
            text,
            mode: mode.clone(),
            start_time: None,
            duration_limit,
            word_limit,
        })
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_mode(&self) -> &TestMode {
        &self.mode
    }

    pub fn is_complete(&self) -> bool {
        if let Some(duration_limit) = self.duration_limit {
            if self.elapsed_time() >= duration_limit {
                return true;
            }
        }
        false
    }

    pub fn elapsed_time(&self) -> Duration {
        if let Some(start) = self.start_time {
            start.elapsed()
        } else {
            Duration::from_secs(0)
        }
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        if let Some(duration_limit) = self.duration_limit {
            let elapsed = self.elapsed_time();
            if elapsed < duration_limit {
                Some(duration_limit - elapsed)
            } else {
                Some(Duration::from_secs(0))
            }
        } else {
            None
        }
    }

    fn generate_text_for_duration(seconds: u32, _config: &Config) -> Result<String, Box<dyn std::error::Error>> {
        let estimated_wpm = 40.0;
        let minutes = seconds as f64 / 60.0;
        let estimated_words = (estimated_wpm * minutes * 1.2) as usize;
        Ok(utils::generate_random_words(estimated_words.max(50))) 
    }
}

// Sample texts
pub const SAMPLE_TEXTS: &[&str] = &[
    "The quick brown fox jumps over the lazy dog. This pangram contains every letter of the alphabet at least once, making it perfect for typing practice.",
    "In a hole in the ground there lived a hobbit. Not a nasty, dirty, wet hole, filled with the ends of worms and an oozy smell, nor yet a dry, bare, sandy hole with nothing in it to sit down on or to eat.",
    "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity.",
    "Call me Ishmael. Some years ago never mind how long precisely having little or no money in my purse, and nothing particular to interest me on shore, I thought I would sail about a little.",
    "To be or not to be, that is the question: Whether 'tis nobler in the mind to suffer the slings and arrows of outrageous fortune, or to take arms against a sea of troubles.",
];

pub fn load_sample_text(index: usize) -> String {
    SAMPLE_TEXTS.get(index).unwrap_or(&SAMPLE_TEXTS[0]).to_string()
}

pub fn get_random_sample_text() -> String {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    SAMPLE_TEXTS.choose(&mut rng).unwrap_or(&SAMPLE_TEXTS[0]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_word_count_mode() {
        let config = Config::default();
        let mode = TestMode::WordCount(10);
        let test = Test::new(&mode, &config).unwrap();
        let word_count = test.get_text().split_whitespace().count();
        assert_eq!(word_count, 10);
    }

    #[test]
    fn test_timed_mode() {
        let config = Config::default();
        let mode = TestMode::Timed(60);
        let test = Test::new(&mode, &config).unwrap();
        assert!(test.duration_limit.is_some());
        assert_eq!(test.duration_limit.unwrap(), Duration::from_secs(60));
    }

    #[test]
    fn test_custom_text_mode() {
        let config = Config::default();
        let custom_text = "This is a custom test text.".to_string();
        let mode = TestMode::Text(custom_text.clone());
        let test = Test::new(&mode, &config).unwrap();
        
        assert_eq!(test.get_text(), &custom_text);
        assert!(test.duration_limit.is_none());
        assert!(test.word_limit.is_none());
    }

    #[test]
    fn test_elapsed_time() {
        let config = Config::default();
        let mode = TestMode::WordCount(5);
        let mut test = Test::new(&mode, &config).unwrap();
        assert_eq!(test.elapsed_time(), Duration::from_secs(0));
        test.start();
        std::thread::sleep(Duration::from_millis(10));
        assert!(test.elapsed_time() > Duration::from_secs(0));
    }
}