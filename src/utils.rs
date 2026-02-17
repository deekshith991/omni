// src/utils.rs

use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// Expands ~ to the user's home directory
pub fn expand_tilde(path: &str) -> String {
    if path.starts_with("~") {
        if let Some(home) = env::var_os("HOME") {
            return path.replacen("~", home.to_str().unwrap(), 1);
        }
    }
    path.to_string()
}

/// Ensures a directory exists, creates it if missing
pub fn ensure_dir_exists(path: &str) -> io::Result<()> {
    let dir = Path::new(path);
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}
