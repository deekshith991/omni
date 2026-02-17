// src/package_manager.rs

use colorize::AnsiColor;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use which::which;

use crate::utils::expand_tilde;

/// Supported package managers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Pacman,
}

/// Detects the system's default package manager
pub fn detect_package_manager() -> Result<PackageManager, &'static str> {
    if which("apt").is_ok() {
        Ok(PackageManager::Apt)
    } else if which("dnf").is_ok() {
        Ok(PackageManager::Dnf)
    } else if which("pacman").is_ok() {
        Ok(PackageManager::Pacman)
    } else {
        Err("No supported package manager found (apt, dnf, pacman)")
    }
}

/// Adds a package entry to the omni.toml configuration file
pub fn add_package(pm: &str, package: &str, comment: Option<&str>) -> std::io::Result<()> {
    let base_path = expand_tilde("~/dotfiles/scripts");
    let config_path = Path::new(&base_path).join("omni.toml");

    if !config_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "omni.toml not found. Run init first.",
        ));
    }

    // Read existing file content
    let mut content = String::new();
    {
        let mut file = OpenOptions::new().read(true).open(&config_path)?;
        file.read_to_string(&mut content)?;
    }

    let section_header = format!("[{}]", pm);
    if !content.contains(&section_header) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Section [{}] not found in omni.toml", pm),
        ));
    }

    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let mut new_lines = Vec::new();
    let mut in_section = false;
    let mut inserted = false;

    // Prepare the new entry line
    let entry_line = match comment {
        Some(c) => format!(r#"{package} = ""  # {c}""#),
        None => format!(r#"{package} = ""#),
    };

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Start of the target section
        if trimmed == section_header {
            in_section = true;
            new_lines.push(line.clone());

            // If next line is a new section or EOF, insert immediately
            if i + 1 >= lines.len() || lines[i + 1].trim().starts_with('[') {
                new_lines.push(entry_line.clone());
                inserted = true;
                in_section = false;
            }

            continue;
        }

        // End of section: reached next section header
        if in_section && trimmed.starts_with('[') && !inserted {
            new_lines.push(entry_line.clone());
            inserted = true;
            in_section = false;
        }

        new_lines.push(line.clone());
    }

    // If section is at EOF and not inserted yet
    if in_section && !inserted {
        new_lines.push(entry_line.clone());
    }

    // Write updated content back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&config_path)?;
    file.write_all(new_lines.join("\n").as_bytes())?;

    println!(
        "Added '{}' to [{}]{}",
        package,
        pm.to_string().blue().bold(),
        comment.map_or("".to_string(), |c| format!(" with comment: {}", c))
    );

    Ok(())
}
