// src/commands/init.rs

use colorize::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::utils::{ensure_dir_exists, expand_tilde};

pub fn run() -> std::io::Result<()> {
    println!("{}", "[=] Running init command...".blue().bold());

    // Expand ~/ to full home directory
    let base_path = expand_tilde("~/dotfiles/scripts");
    let config_path = Path::new(&base_path).join("omni.toml");

    // Ensure directory exists
    ensure_dir_exists(&base_path)?;
    println!("Ensured directory exists: {}", base_path.green());

    if !config_path.exists() {
        println!("{}", "omni.toml not found. Creating...".yellow().bold());

        let mut file = File::create(&config_path)?;

        // Default template content
        let default_content = r#"
# omni.toml
[settings]
version = "0"
[apt]
[general]
[dnf]
[pacman]
"#;

        file.write_all(default_content.as_bytes())?;
        println!("{} {:?}", "Created:".green().bold(), config_path);
    } else {
        println!(
            "{} omni.toml already exists at: {:?}",
            "[=]".green(),
            config_path
        );
    }

    Ok(())
}
