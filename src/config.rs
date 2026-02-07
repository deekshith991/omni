use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub general: PackageSection,
    pub apt: PackageSection,
    pub dnf: PackageSection,
    pub pacman: PackageSection,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PackageSection {
    pub packages: Vec<String>,
}

impl Config {
    pub fn load(path: &Path) -> Self {
        if !path.exists() {
            return Config::default();
        }

        let contents = fs::read_to_string(path).expect("Failed to read config");
        toml::from_str(&contents).expect("Failed to parse config")
    }

    pub fn save(&self, path: &Path) {
        let toml_str = toml::to_string_pretty(self).expect("Failed to serialize config");
        fs::write(path, toml_str).expect("Failed to write config");
    }
}
