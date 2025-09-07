use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::stats::Stats;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct History {
    pub results: Vec<Stats>,
}

impl Default for History {
    fn default() -> Self {
        Self {
            results: Vec::new(),
        }
    }
}

impl History {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let history_path = Self::get_history_path()?;
        if history_path.exists() {
            let content = fs::read_to_string(&history_path)?;
            let history: History = serde_json::from_str(&content)?;
            Ok(history)
        } else {
            let default_history = History::default();
            default_history.save()?;
            Ok(default_history)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let history_path = Self::get_history_path()?;
        if let Some(parent) = history_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&history_path, content)?;
        Ok(())
    }

    fn get_history_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::data_dir().or_else(|| dirs::config_dir()).ok_or("Could not find data or config directory")?;
        path.push("typing-test");
        path.push("history.json");
        Ok(path)
    }

    pub fn add_result(&mut self, stats: &Stats) -> Result<(), Box<dyn std::error::Error>> {
        self.results.push(stats.clone());
        if self.results.len() > 1000 {
            self.results.drain(0..self.results.len() - 1000);
        }
        self.results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        self.save()
    }

    pub fn get_results(&self) -> &[Stats] {
        &self.results
    }

    pub fn get_best_wpm(&self) -> Option<f64> {
        self.results.iter().map(|r| r.wpm).fold(None, |max, wpm| {
            Some(max.map_or(wpm, |m| m.max(wpm)))
        })
    }

    pub fn get_best_accuracy(&self) -> Option<f64> {
        self.results.iter().map(|r| r.accuracy).fold(None, |max, acc| {
            Some(max.map_or(acc, |m| m.max(acc)))
        })
    }

    pub fn get_average_wpm(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        self.results.iter().map(|r| r.wpm).sum::<f64>() / self.results.len() as f64
    }

    pub fn get_average_accuracy(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        self.results.iter().map(|r| r.accuracy).sum::<f64>() / self.results.len() as f64
    }

    pub fn get_recent_results(&self, count: usize) -> Vec<&Stats> {
        self.results.iter().take(count).collect()
    }

    pub fn get_results_by_mode(&self, mode_name: &str) -> Vec<&Stats> {
        self.results.iter().filter(|r| r.test_mode == mode_name).collect()
    }

    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.results.clear();
        self.save()
    }

    pub fn get_personal_best(&self) -> Option<&Stats> {
        self.results.iter().max_by(|a, b| a.wpm.partial_cmp(&b.wpm).unwrap_or(std::cmp::Ordering::Equal))
    }

    pub fn get_improvement_over_time(&self) -> Vec<(chrono::DateTime<chrono::Utc>, f64)> {
        let mut results: Vec<_> = self.results.iter().map(|r| (r.timestamp, r.wpm)).collect();
        results.sort_by_key(|(timestamp, _)| *timestamp);
        results
    }

    pub fn get_consistency_trend(&self) -> Vec<(chrono::DateTime<chrono::Utc>, f64)> {
        let mut results: Vec<_> = self.results.iter().map(|r| (r.timestamp, r.consistency_score)).collect();
        results.sort_by_key(|(timestamp, _)| *timestamp);
        results
    }

    pub fn get_accuracy_trend(&self) -> Vec<(chrono::DateTime<chrono::Utc>, f64)> {
        let mut results: Vec<_> = self.results.iter().map(|r| (r.timestamp, r.accuracy)).collect();
        results.sort_by_key(|(timestamp, _)| *timestamp);
        results
    }

    pub fn get_stats_summary(&self) -> HistorySummary {
        HistorySummary {
            total_tests: self.results.len(),
            best_wpm: self.get_best_wpm().unwrap_or(0.0),
            best_accuracy: self.get_best_accuracy().unwrap_or(0.0),
            average_wpm: self.get_average_wpm(),
            average_accuracy: self.get_average_accuracy(),
            total_time_spent: self.results.iter().map(|r| r.test_duration.as_secs()).sum::<u64>(),
            most_common_mode: self.get_most_common_mode(),
        }
    }

    fn get_most_common_mode(&self) -> String {
        let mut mode_counts = HashMap::new();
        for result in &self.results {
            *mode_counts.entry(result.test_mode.clone()).or_insert(0) += 1;
        }
        mode_counts.into_iter().max_by_key(|(_, count)| *count).map(|(mode, _)| mode).unwrap_or_else(|| "None".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct HistorySummary {
    pub total_tests: usize,
    pub best_wpm: f64,
    pub best_accuracy: f64,
    pub average_wpm: f64,
    pub average_accuracy: f64,
    pub total_time_spent: u64,
    pub most_common_mode: String,
}

impl HistorySummary {
    pub fn format_total_time(&self) -> String {
        let hours = self.total_time_spent / 3600;
        let minutes = (self.total_time_spent % 3600) / 60;
        let seconds = self.total_time_spent % 60;
        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::Stats;
    use chrono::Utc;
    use std::time::Duration;
    use std::collections::HashMap;

    #[test]
    fn test_history_creation() {
        let history = History::default();
        assert_eq!(history.results.len(), 0);
    }

    #[test]
    fn test_add_result() {
        let mut history = History::default();
        let stats = Stats {
            timestamp: Utc::now(),
            test_mode: "Test".to_string(),
            wpm: 50.0,
            raw_wpm: 55.0,
            accuracy: 0.95,
            error_count: 5,
            correct_chars: 95,
            total_chars: 100,
            test_duration: Duration::from_secs(60),
            error_frequency: HashMap::new(),
            speed_over_time: vec![(0.0, 0.0)],
            consistency_score: 0.8,
        };
        
        history.add_result(&stats).unwrap();
        assert_eq!(history.results.len(), 1);
        assert_eq!(history.get_best_wpm(), Some(50.0));
    }

    #[test]
    fn test_statistics() {
        let mut history = History::default();
        
        let stats1 = Stats {
            timestamp: Utc::now(),
            test_mode: "Test".to_string(),
            wpm: 40.0,
            raw_wpm: 45.0,
            accuracy: 0.90,
            error_count: 10,
            correct_chars: 90,
            total_chars: 100,
            test_duration: Duration::from_secs(60),
            error_frequency: HashMap::new(),
            speed_over_time: vec![(0.0, 0.0)],
            consistency_score: 0.7,
        };

        let stats2 = Stats {
            wpm: 60.0,
            accuracy: 0.95,
            ..stats1.clone()
        };
        history.add_result(&stats1).unwrap();
        history.add_result(&stats2).unwrap();
        assert_eq!(history.get_best_wpm(), Some(60.0));
        assert_eq!(history.get_average_wpm(), 50.0);
        assert_eq!(history.get_best_accuracy(), Some(0.95));
    }
}