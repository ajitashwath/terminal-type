use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{config::Config, history::History, input::InputHandler, stats::Stats, test::Test};

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
            TestMode::WordCount(words) => format!("Word Count ({})", words),
            TestMode::Text(_) => "Custom Text".to_string(),
        }
    }
}

pub struct App {
    pub should_quit: bool,
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

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load().map_err(|e| anyhow::anyhow!("{}", e))?;
        let history = History::load().map_err(|e| anyhow::anyhow!("{}", e))?;

        let available_modes = vec![
            TestMode::Timed(30),
            TestMode::Timed(60),
            TestMode::Timed(120),
            TestMode::WordCount(25),
            TestMode::WordCount(50),
            TestMode::WordCount(100),
        ];

        let menu_items = vec![
            "Start Test".to_string(),
            "Change Mode".to_string(),
            "View History".to_string(),
            "Quit".to_string(),
        ];

        Ok(Self {
            should_quit: false,
            current_screen: Screen::Menu,
            config,
            history,
            selected_menu_item: 0,
            menu_items,
            test: None,
            input_handler: InputHandler::new(),
            current_mode: available_modes[0].clone(),
            available_modes,
            selected_mode_index: 0,
            last_stats: None,
            selected_history_item: 0,
        })
    }

    pub fn can_quit(&self) -> bool {
        matches!(self.current_screen, Screen::Menu | Screen::Results | Screen::History)
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match self.current_screen {
            Screen::Menu => self.handle_menu_key(key),
            Screen::Test => self.handle_test_key(key),
            Screen::Results => self.handle_results_key(key),
            Screen::History => self.handle_history_key(key),
        }
    }

    fn handle_menu_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_menu_item > 0 {
                    self.selected_menu_item -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_menu_item < self.menu_items.len() - 1 {
                    self.selected_menu_item += 1;
                }
            }
            KeyCode::Enter => {
                match self.selected_menu_item {
                    0 => self.start_test()?,
                    1 => self.show_mode_selection(),
                    2 => self.show_history(),
                    3 => self.should_quit = true,
                    _ => {}
                }
            }
            KeyCode::Char('1') => self.start_test()?,
            KeyCode::Char('2') => self.show_mode_selection(),
            KeyCode::Char('3') => self.show_history(),
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
        Ok(())
    }

    fn handle_test_key(&mut self, key: KeyEvent) -> Result<()> {
        if key.code == KeyCode::Esc {
            self.current_screen = Screen::Menu;
            self.test = None;
            return Ok(());
        }
        if let Some(test) = &mut self.test {
            self.input_handler.handle_key(key, test).map_err(|e| anyhow::anyhow!("{}", e))?;
            if test.is_complete() {
                let stats = Stats::calculate(test, &self.input_handler);
                self.history.add_result(&stats).map_err(|e| anyhow::anyhow!("{}", e))?;
                self.last_stats = Some(stats);
                self.current_screen = Screen::Results;
                self.test = None;
            }
        }
        Ok(())
    }

    fn handle_results_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.start_test()?;
            }
            KeyCode::Char('m') | KeyCode::Char('M') | KeyCode::Esc => {
                self.current_screen = Screen::Menu;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_history_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_history_item > 0 {
                    self.selected_history_item -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max_items = self.history.get_results().len();
                if max_items > 0 && self.selected_history_item < max_items - 1 {
                    self.selected_history_item += 1;
                }
            }
            KeyCode::Char('m') | KeyCode::Esc => {
                self.current_screen = Screen::Menu;
            }
            _ => {}
        }
        Ok(())
    }

    fn start_test(&mut self) -> Result<()> {
        self.test = Some(Test::new(&self.current_mode, &self.config).map_err(|e| anyhow::anyhow!("{}", e))?);
        self.input_handler = InputHandler::new();
        self.current_screen = Screen::Test;
        Ok(())
    }

    fn show_mode_selection(&mut self) {
        self.selected_mode_index = (self.selected_mode_index + 1) % self.available_modes.len();
        self.current_mode = self.available_modes[self.selected_mode_index].clone();
    }

    fn show_history(&mut self) {
        self.current_screen = Screen::History;
        self.selected_history_item = 0;
    }
}