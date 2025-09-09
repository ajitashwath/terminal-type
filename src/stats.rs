use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::{input::InputHandler, test::Test};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub timestamp: DateTime<Utc>,
    pub test_mode: String,
    pub wpm: f64,
    pub raw_wpm: f64,
    pub accuracy: f64,
    pub error_count: usize,
    pub correct_chars: usize,
    pub total_chars: usize,
    pub test_duration: Duration,
    pub error_frequency: HashMap<char, usize>,
    pub speed_over_time: Vec<(f64, f64)>,
    pub consistency_score: f64,
}

#[derive(Debug, Clone)]
pub struct LiveStats {
    pub wpm: f64,
    pub accuracy: f64,
    pub error_count: usize,
}

impl Stats {
    pub fn calculate(test: &Test, input_handler: &InputHandler) -> Self {
        let now = Utc::now();
        let test_duration = test.elapsed_time();
        let elapsed_minutes = test_duration.as_secs_f64() / 60.0;

        let typing_keystrokes: Vec<_> = input_handler.get_keystrokes().iter().filter(|k| !k.is_correction).collect();
        let total_chars = typing_keystrokes.len();
        let correct_chars = typing_keystrokes.iter().filter(|k| k.is_correct).count();
        let error_count = total_chars - correct_chars;

        let accuracy = if total_chars > 0 {
            correct_chars as f64 / total_chars as f64
        } else {
            1.0
        };

        let (wpm, raw_wpm) = if elapsed_minutes > 0.0 {
            let raw_wpm = (total_chars as f64 / 5.0) / elapsed_minutes;
            let net_wpm = (correct_chars as f64 / 5.0) / elapsed_minutes;
            (net_wpm, raw_wpm)
        } else {
            (0.0, 0.0)
        };

        let error_frequency = input_handler.calculate_error_frequency(test.get_text());
        let speed_over_time = input_handler.get_speed_over_time();
        let consistency_score = input_handler.get_consistency_score();
        Stats {
            timestamp: now,
            test_mode: test.get_mode().display_name(),
            wpm,
            raw_wpm,
            accuracy,
            error_count,
            correct_chars,
            total_chars,
            test_duration,
            error_frequency,
            speed_over_time,
            consistency_score,
        }
    }

    pub fn get_grade(&self) -> &'static str {
        match self.wpm {
            wpm if wpm >= 80.0 => "Expert",
            wpm if wpm >= 60.0 => "Advanced",
            wpm if wpm >= 40.0 => "Intermediate", 
            wpm if wpm >= 25.0 => "Beginner",
            _ => "Learning",
        }
    }

    pub fn get_accuracy_grade(&self) -> &'static str {
        match self.accuracy {
            acc if acc >= 0.98 => "Excellent",
            acc if acc >= 0.95 => "Good",
            acc if acc >= 0.90 => "Fair",
            acc if acc >= 0.80 => "Needs Work",
            _ => "Poor",
        }
    }

    pub fn get_summary(&self) -> String {
        format!(
            "{:.0} WPM ({:.1}% accuracy) - {} typing, {} accuracy",
            self.wpm,
            self.accuracy * 100.0,
            self.get_grade(),
            self.get_accuracy_grade()
        )
    }

    pub fn calculate_improvement(&self, previous: &Stats) -> ImprovementStats {
        ImprovementStats {
            wpm_change: self.wpm - previous.wpm,
            accuracy_change: self.accuracy - previous.accuracy,
            error_count_change: self.error_count as i32 - previous.error_count as i32,
            consistency_change: self.consistency_score - previous.consistency_score,
        }
    }

    pub fn get_strongest_fingers(&self) -> Vec<(String, f64)> {
        let finger_keys = vec![
            ("Left Pinky", vec!['q', 'a', 'z', '1', '!', '\t']),
            ("Left Ring", vec!['w', 's', 'x', '2', '@']),
            ("Left Middle", vec!['e', 'd', 'c', '3', '#']),
            ("Left Index", vec!['r', 'f', 'v', 't', 'g', 'b', '4', '$', '5', '%']),
            ("Right Index", vec!['y', 'h', 'n', 'u', 'j', 'm', '6', '^', '7', '&']),
            ("Right Middle", vec!['i', 'k', ',', '8', '*']),
            ("Right Ring", vec!['o', 'l', '.', '9', '(']),
            ("Right Pinky", vec!['p', ';', '/', '[', ']', '\'', '0', ')', '-', '_', '=', '+']),
        ];

        let mut finger_scores = Vec::new();
        for (finger_name, keys) in finger_keys {
            let total_errors: usize = keys.iter().map(|&key| self.error_frequency.get(&key).unwrap_or(&0)).sum();
            let total_opportunities: usize = keys.len();
            let accuracy = if total_opportunities > 0 {
                1.0 - (total_errors as f64 / total_opportunities as f64)
            } else {
                1.0
            };

            finger_scores.push((finger_name.to_string(), accuracy));
        }
        finger_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        finger_scores
    }

    pub fn get_problem_characters(&self) -> Vec<(char, usize)> {
        let mut errors: Vec<_> = self.error_frequency.iter().map(|(&ch, &count)| (ch, count)).collect();
        errors.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
        errors.into_iter().take(10).collect()
    }

    pub fn estimate_time_to_goal(&self, target_wpm: f64) -> Option<String> {
        if self.wpm >= target_wpm {
            return Some("Goal already achieved!".to_string());
        }

        let wpm_gap = target_wpm - self.wpm;
        let improvement_per_week = 2.0;
        let weeks_needed = wpm_gap / improvement_per_week;
        
        if weeks_needed < 1.0 {
            Some(format!("{:.0} days", weeks_needed * 7.0))
        } else if weeks_needed < 4.0 {
            Some(format!("{:.1} weeks", weeks_needed))
        } else {
            Some(format!("{:.1} months", weeks_needed / 4.0))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImprovementStats {
    pub wpm_change: f64,
    pub accuracy_change: f64,
    pub error_count_change: i32,
    pub consistency_change: f64,
}

impl ImprovementStats {
    pub fn get_summary(&self) -> Vec<String> {
        let mut summary = Vec::new();
        if self.wpm_change > 0.5 {
            summary.push(format!("WPM improved by {:.1}", self.wpm_change));
        } else if self.wpm_change < -0.5 {
            summary.push(format!("WPM decreased by {:.1}", -self.wpm_change));
        } else {
            summary.push("WPM remained stable".to_string());
        }

        if self.accuracy_change > 0.01 {
            summary.push(format!("Accuracy improved by {:.1}%", self.accuracy_change * 100.0));
        } else if self.accuracy_change < -0.01 {
            summary.push(format!("Accuracy decreased by {:.1}%", -self.accuracy_change * 100.0));
        } else {
            summary.push("Accuracy remained stable".to_string());
        }

        if self.error_count_change < 0 {
            summary.push(format!("Errors reduced by {}", -self.error_count_change));
        } else if self.error_count_change > 0 {
            summary.push(format!("Errors increased by {}", self.error_count_change));
        }

        if self.consistency_change > 0.05 {
            summary.push("Typing became more consistent".to_string());
        } else if self.consistency_change < -0.05 {
            summary.push("Typing became less consistent".to_string());
        }
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stats_calculation() {
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
            speed_over_time: vec![(0.0, 0.0), (60.0, 50.0)],
            consistency_score: 0.8,
        };
        assert_eq!(stats.get_grade(), "Intermediate");
        assert_eq!(stats.get_accuracy_grade(), "Good");
    }

    #[test]
    fn test_improvement_calculation() {
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
            wpm: 45.0,
            accuracy: 0.95,
            error_count: 5,
            consistency_score: 0.8,
            ..stats1.clone()
        };

        let improvement = stats2.calculate_improvement(&stats1);
        assert_eq!(improvement.wpm_change, 5.0);
        assert_eq!(improvement.accuracy_change, 0.05);
        assert_eq!(improvement.error_count_change, -5);
        assert_eq!(improvement.consistency_change, 0.1);
    }
}