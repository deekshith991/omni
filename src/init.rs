// src/init.rs

use std::path::{Path, PathBuf};

use crate::util;

pub fn run(_dir: &str) -> PathBuf {
    // ALWAYS use this directory
    const DEFAULT_DIR: &str = "~/dotfiles/scripts";

    let path = util::resolve_path(DEFAULT_DIR);

    // Ensure directory exists (kept this in case you still want it)
    util::ensure_directory_exists(&path);

    let absolute_path = util::get_absolute_path(&path);

    println!("Initializing Omni in {}", absolute_path.display());

    create_config_if_missing(&absolute_path);

    absolute_path
}

fn create_config_if_missing(path: &Path) {
    let config_path = path.join("omni.toml");

    if config_path.exists() {
        println!("Config already exists at {}", config_path.display());
    } else {
        std::fs::write(&config_path, get_base_config()).expect("Failed to create config file");

        println!("Created config: {}", config_path.display());
    }
}

fn get_base_config() -> &'static str {
    r#"
[apt]
packages = []

[dnf]
packages = []

[general]
packages = []

[pacman]
packages = []

"#
}
