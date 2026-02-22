use std::fs;
use std::path::Path;

use colorize::*;
use toml_edit::{Document, Item};
use crate::utils::expand_tilde;

pub fn run() -> std::io::Result<()> {
    println!("{}", "[=] Listing packages...".blue().bold());

    let base_path = expand_tilde("~/dotfiles/scripts");
    let config_path = Path::new(&base_path).join("omni.toml");

    if !config_path.exists() {
        eprintln!("{}", "omni.toml not found. Run init first.".red().bold());
        return Ok(());
    }

    // Read and parse the TOML file
    let content = fs::read_to_string(&config_path)?;
    let doc: Document<String> = content
        .parse::<Document<String>>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Print version safely
    if let Some(Item::Table(settings_table)) = doc.get("settings")
        && let Some(version) = settings_table
            .get("version")
            .and_then(|v| v.as_value())
            .and_then(|v| v.as_str())
        {
            let version = version.to_string(); // clone to break borrow
            println!("{} {}", "[•] Version:".cyan().bold(), version.bold());
        }


    // Print package sections
    for section_name in &["general", "apt", "dnf", "pacman"] {
        print_section(&doc, section_name);
    }

    Ok(())
}

fn print_section(doc: &Document<String>, section_name: &str) {
    if let Some(Item::Table(table)) = doc.get(section_name) {
        if table.is_empty() {
            return;
        }

        println!("\n{} {}", "[•]".green().bold(), section_name.to_string().bold());

        let mut keys: Vec<_> = table.iter().collect();
        keys.sort_by(|a, b| a.0.cmp(b.0));

        for (key, item) in keys {
            if let Item::Value(value) = item {
                let comment = value
                    .decor()
                    .prefix()
                    .and_then(|s| s.as_str())
                    .map(|s| s.trim().trim_start_matches('#').to_string())
                    .unwrap_or_default();

                if comment.is_empty() {
                    println!("   - {}", key);
                } else {
                    println!("   - {} ({})", key, comment.cyan());
                }
            } else {
                println!("   - {}", key);
            }
        }

        println!("   {} {} packages", "→".cyan(), table.len());
    }
}