use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub test_settings: TestSettings,
    pub keybindings: Keybindings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub text: SerializableColor,
    pub background: SerializableColor,
    pub accent: SerializableColor,
    pub correct: SerializableColor,
    pub error: SerializableColor,
    pub cursor: SerializableColor,
    pub border: SerializableColor,
    pub highlight: SerializableColor,
    pub muted: SerializableColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<SerializableColor> for Color {
    fn from(color: SerializableColor) -> Self {
        Color::Rgb(color.r, color.g, color.b)
    }
}

impl From<Color> for SerializableColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Rgb(r, g, b) => SerializableColor { r, g, b },
            Color::Black => SerializableColor { r: 0, g: 0, b: 0 },
            Color::Red => SerializableColor { r: 255, g: 0, b: 0 },
            Color::Green => SerializableColor { r: 0, g: 255, b: 0 },
            Color::Yellow => SerializableColor { r: 255, g: 255, b: 0 },
            Color::Blue => SerializableColor { r: 0, g: 0, b: 255 },
            Color::Magenta => SerializableColor { r: 255, g: 0, b: 255 },
            Color::Cyan => SerializableColor { r: 0, g: 255, b: 255 },
            Color::Gray => SerializableColor { r: 128, g: 128, b: 128 },
            Color::DarkGray => SerializableColor { r: 64, g: 64, b: 64 },
            Color::LightRed => SerializableColor { r: 255, g: 128, b: 128 },
            Color::LightGreen => SerializableColor { r: 128, g: 255, b: 128 },
            Color::LightYellow => SerializableColor { r: 255, g: 255, b: 128 },
            Color::LightBlue => SerializableColor { r: 128, g: 128, b: 255 },
            Color::LightMagenta => SerializableColor { r: 255, g: 128, b: 255 },
            Color::LightCyan => SerializableColor { r: 128, g: 255, b: 255 },
            Color::White => SerializableColor { r: 255, g: 255, b: 255 },
            Color::Reset => SerializableColor { r: 255, g: 255, b: 255 },
            _ => SerializableColor { r: 255, g: 255, b: 255 },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSettings {
    pub default_mode: String,
    pub default_duration: u32,
    pub default_word_count: u32,
    pub sound_enabled: bool,
    pub show_live_wpm: bool,
    pub show_live_accuracy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybindings {
    pub quit: String,
    pub restart: String,
    pub menu: String,
    pub next_mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: Theme::default_dark(),
            test_settings: TestSettings::default(),
            keybindings: Keybindings::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let default_config = Config::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        path.push("typing-test");
        path.push("config.toml");
        Ok(path)
    }
}

impl Theme {
    pub fn default_dark() -> Self {
        Theme {
            name: "Dark".to_string(),
            text: Color::White.into(),
            background: Color::Black.into(),
            accent: Color::Cyan.into(),
            correct: Color::Green.into(),
            error: Color::Red.into(),
            cursor: Color::Yellow.into(),
            border: Color::Gray.into(),
            highlight: Color::LightCyan.into(),
            muted: Color::DarkGray.into(),
        }
    }

    pub fn default_light() -> Self {
        Theme {
            name: "Light".to_string(),
            text: Color::Black.into(),
            background: Color::White.into(),
            accent: Color::Blue.into(),
            correct: Color::Green.into(),
            error: Color::Red.into(),
            cursor: Color::Magenta.into(),
            border: Color::Gray.into(),
            highlight: Color::LightBlue.into(),
            muted: Color::Gray.into(),
        }
    }

    pub fn monokai() -> Self {
        Theme {
            name: "Monokai".to_string(),
            text: SerializableColor { r: 248, g: 248, b: 242 },
            background: SerializableColor { r: 39, g: 40, b: 34 },
            accent: SerializableColor { r: 166, g: 226, b: 46 },
            correct: SerializableColor { r: 166, g: 226, b: 46 },
            error: SerializableColor { r: 249, g: 38, b: 114 },
            cursor: SerializableColor { r: 253, g: 151, b: 31 },
            border: SerializableColor { r: 117, g: 113, b: 94 },
            highlight: SerializableColor { r: 102, g: 217, b: 239 },
            muted: SerializableColor { r: 117, g: 113, b: 94 },
        }
    }

    pub fn solarized_dark() -> Self {
        Theme {
            name: "Solarized Dark".to_string(),
            text: SerializableColor { r: 131, g: 148, b: 150 },
            background: SerializableColor { r: 0, g: 43, b: 54 },
            accent: SerializableColor { r: 42, g: 161, b: 152 },
            correct: SerializableColor { r: 133, g: 153, b: 0 },
            error: SerializableColor { r: 220, g: 50, b: 47 },
            cursor: SerializableColor { r: 181, g: 137, b: 0 },
            border: SerializableColor { r: 88, g: 110, b: 117 },
            highlight: SerializableColor { r: 38, g: 139, b: 210 },
            muted: SerializableColor { r: 88, g: 110, b: 117 },
        }
    }

    pub fn dracula() -> Self {
        Theme {
            name: "Dracula".to_string(),
            text: SerializableColor { r: 248, g: 248, b: 242 },
            background: SerializableColor { r: 40, g: 42, b: 54 },
            accent: SerializableColor { r: 189, g: 147, b: 249 },
            correct: SerializableColor { r: 80, g: 250, b: 123 },
            error: SerializableColor { r: 255, g: 85, b: 85 },
            cursor: SerializableColor { r: 255, g: 184, b: 108 },
            border: SerializableColor { r: 68, g: 71, b: 90 },
            highlight: SerializableColor { r: 139, g: 233, b: 253 },
            muted: SerializableColor { r: 98, g: 114, b: 164 },
        }
    }

    pub fn get_available_themes() -> Vec<Theme> {
        vec![
            Theme::default_dark(),
            Theme::default_light(),
            Theme::monokai(),
            Theme::solarized_dark(),
            Theme::dracula(),
        ]
    }
}

impl Theme {
    pub fn text(&self) -> Color { self.text.clone().into() }
    pub fn background(&self) -> Color { self.background.clone().into() }
    pub fn accent(&self) -> Color { self.accent.clone().into() }
    pub fn correct(&self) -> Color { self.correct.clone().into() }
    pub fn error(&self) -> Color { self.error.clone().into() }
    pub fn cursor(&self) -> Color { self.cursor.clone().into() }
    pub fn border(&self) -> Color { self.border.clone().into() }
    pub fn highlight(&self) -> Color { self.highlight.clone().into() }
    pub fn muted(&self) -> Color { self.muted.clone().into() }
}

impl Default for TestSettings {
    fn default() -> Self {
        TestSettings {
            default_mode: "Timed30".to_string(),
            default_duration: 30,
            default_word_count: 25,
            sound_enabled: false,
            show_live_wpm: true,
            show_live_accuracy: true,
        }
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        Keybindings {
            quit: "q".to_string(),
            restart: "r".to_string(),
            menu: "m".to_string(),
            next_mode: "Tab".to_string(),
        }
    }
}

pub fn create_sample_config() -> String {
    let config = Config::default();
    toml::to_string_pretty(&config).unwrap_or_else(|_| {
        r#"[theme]
name = "Dark"

[theme.text]
r = 255
g = 255
b = 255

[theme.background]
r = 0
g = 0
b = 0

[theme.accent]
r = 0
g = 255
b = 255

[theme.correct]
r = 0
g = 255
b = 0

[theme.error]
r = 255
g = 0
b = 0

[theme.cursor]
r = 255
g = 255
b = 0

[theme.border]
r = 128
g = 128
b = 128

[theme.highlight]
r = 128
g = 255
b = 255

[theme.muted]
r = 64
g = 64
b = 64

[test_settings]
default_mode = "Timed30"
default_duration = 30
default_word_count = 25
sound_enabled = false
show_live_wpm = true
show_live_accuracy = true

[keybindings]
quit = "q"
restart = "r"
menu = "m"
next_mode = "Tab"
"#.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.theme.name, "Dark");
        assert_eq!(config.test_settings.default_duration, 30);
        assert_eq!(config.keybindings.quit, "q");
    }

    #[test]
    fn test_color_conversion() {
        let color = Color::Red;
        let serializable: SerializableColor = color.into();
        let converted: Color = serializable.into();
        assert_eq!(serializable.r, 255);
        assert_eq!(serializable.g, 0);
        assert_eq!(serializable.b, 0);
    }

    #[test]
    fn test_theme_creation() {
        let theme = Theme::monokai();
        assert_eq!(theme.name, "Monokai");
        
        let themes = Theme::get_available_themes();
        assert!(themes.len() >= 4);
    }
}