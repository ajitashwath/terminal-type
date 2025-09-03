use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::Instant;

use crate::{config::Config, history::History, input::InputHandler, stats::Stats, test::Test};

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Menu,
    Test,
    Results,
    History,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestMode {
    Timed(u32),
    WordCount(u32),
    Text(String),
}

impl TestMode {
    pub fn display_name(&self) -> String {
        match self {
            TestMode::Timed(secs) => format!("Timed ({}s)", secs),
            TestMode::WordCunt(words) => format!("Word Count ({})", words),
            TestMode::Text(_) => "Custom Text".to_string(),
        }
    }
}

pub struct App {
    pub should_quit:bool,
    pub current_screen: Screen,
    pub config: Config,
    pub history: History,

    pub selected_menu_item: usize,
    pub menu_items: Vec<String>,

    pub test: Option<Test>,
    pub input_handler: InputHandler,
    pub current_mode: TestMode,
    pub available_modes: Vec<TestMode>,
    pub selected_mode_index: usize,
    pub last_stats: Option<Stats>,
    pub selected_history_item: usize,
}
