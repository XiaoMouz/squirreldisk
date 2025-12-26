use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_config_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IgnorePattern {
    pub pattern: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IgnoreConfig {
    pub patterns: Vec<IgnorePattern>,
}

impl Default for IgnoreConfig {
    fn default() -> Self {
        Self {
            patterns: vec![],
        }
    }
}

impl IgnoreConfig {
    pub fn load(config: &tauri::Config) -> Result<Self, String> {
        let config_path = Self::get_config_path(config)?;
        
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }

    pub fn save(&self, config: &tauri::Config) -> Result<(), String> {
        let config_path = Self::get_config_path(config)?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(&config_path, contents)
            .map_err(|e| format!("Failed to write config: {}", e))
    }

    fn get_config_path(config: &tauri::Config) -> Result<PathBuf, String> {
        let config_dir = app_config_dir(config)
            .ok_or_else(|| "Failed to get app config directory".to_string())?;
        Ok(config_dir.join("ignore_patterns.json"))
    }

    pub fn should_ignore(&self, path: &str) -> bool {
        for pattern in &self.patterns {
            if !pattern.enabled {
                continue;
            }

            // Check if path matches the pattern
            if Self::matches_pattern(path, &pattern.pattern) {
                return true;
            }
        }
        false
    }

    fn matches_pattern(path: &str, pattern: &str) -> bool {
        // Normalize path separators
        let normalized_path = path.replace('\\', "/");
        
        // Check exact match
        if normalized_path.ends_with(&pattern.replace('\\', "/")) {
            return true;
        }

        // Check if any path component matches
        for component in normalized_path.split('/') {
            if Self::glob_match(component, pattern) {
                return true;
            }
        }

        // Check full path glob match
        Self::glob_match(&normalized_path, pattern)
    }

    fn glob_match(text: &str, pattern: &str) -> bool {
        // Simple glob matching for *, ?, and ** patterns
        let mut text_chars = text.chars().peekable();
        let mut pattern_chars = pattern.chars().peekable();

        while let Some(&p) = pattern_chars.peek() {
            match p {
                '*' => {
                    pattern_chars.next();
                    if pattern_chars.peek().is_none() {
                        return true; // * at end matches everything
                    }
                    
                    // Try to match the rest of the pattern at different positions
                    while text_chars.peek().is_some() {
                        let remaining_text: String = text_chars.clone().collect();
                        let remaining_pattern: String = pattern_chars.clone().collect();
                        if Self::glob_match(&remaining_text, &remaining_pattern) {
                            return true;
                        }
                        text_chars.next();
                    }
                    return false;
                }
                '?' => {
                    pattern_chars.next();
                    if text_chars.next().is_none() {
                        return false;
                    }
                }
                _ => {
                    pattern_chars.next();
                    if text_chars.next() != Some(p) {
                        return false;
                    }
                }
            }
        }

        text_chars.peek().is_none()
    }
}
