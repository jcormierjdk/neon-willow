//! Core library functions for neon_willow.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub trim: bool,
    pub uppercase: bool,
    pub max_lines: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            trim: true,
            uppercase: false,
            max_lines: None,
        }
    }
}

pub fn process_line(line: &str, config: &Config) -> String {
    let mut result = line.to_string();
    if config.trim {
        result = result.trim().to_string();
    }
    if config.uppercase {
        result = result.to_uppercase();
    }
    result
}