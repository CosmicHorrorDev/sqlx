use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    pub merged_strategy: MergedStrategy,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum MergedStrategy {
    Clean,
    Touch(TouchStrategy),
}

impl Default for MergedStrategy {
    fn default() -> Self {
        Self::Clean
    }
}

#[derive(Deserialize)]
pub struct TouchStrategy {
    pub packages_to_clean: Vec<String>,
}

impl Config {
    pub fn from_path(path: &Path) -> crate::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(Into::into)
    }
}
