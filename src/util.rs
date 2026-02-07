// src/util.rs

use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve_path(dir: &str) -> PathBuf {
    if dir.starts_with("~") {
        let home = home_dir().expect("Could not find home directory");
        let without_tilde = dir.trim_start_matches("~").trim_start_matches("/");
        home.join(without_tilde)
    } else {
        PathBuf::from(dir)
    }
}

pub fn ensure_directory_exists(path: &Path) {
    if path.exists() && !path.is_dir() {
        panic!("Path exists but is not a directory");
    }

    if !path.exists() {
        println!("Directory does not exist. Creating...");
        fs::create_dir_all(path).expect("Failed to create directory");
    }
}

pub fn get_absolute_path(path: &Path) -> PathBuf {
    fs::canonicalize(path).expect("Failed to canonicalize path")
}

// New: get config path
pub fn get_config_path() -> PathBuf {
    let base = resolve_path("~/dotfiles/scripts");
    base.join("omni.toml")
}
