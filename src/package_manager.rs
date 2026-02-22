// src/package_manager.rs

use colorize::AnsiColor;
use std::fs;
use std::io;
use std::path::Path;
use which::which;

use crate::utils::expand_tilde;
use toml_edit::{DocumentMut, value};

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
pub fn add_package(pm: &str, package: &str, comment: Option<&str>) -> io::Result<()> {
    let base_path = expand_tilde("~/dotfiles/scripts");
    let config_path = Path::new(&base_path).join("omni.toml");

    if !config_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "omni.toml not found. Run init first.",
        ));
    }

    // Read and parse TOML
    let content = fs::read_to_string(&config_path)?;
    let mut doc = content
        .parse::<DocumentMut>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Ensure section exists
    if !doc.as_table().contains_key(pm) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Section [{}] not found in omni.toml", pm),
        ));
    }

    let table = doc[pm]
        .as_table_mut()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid section type"))?;

    // Prevent duplicate entries
    if table.contains_key(package) {
        println!(
            "{} '{}' already exists in [{}]",
            "[!]".yellow().bold(),
            package,
            pm.to_string().blue().bold()
        );
        return Ok(());
    }

    // Insert new package
    table[package] = value("");

    // Add optional comment
    if let Some(c) = comment
        && let Some(value) = table[package].as_value_mut()
    {
        value.decor_mut().set_suffix(format!("  # {}", c));
    }

    // Write updated TOML back
    fs::write(&config_path, doc.to_string())?;

    println!(
        "{} '{}' added to [{}]{}",
        "[+]".green().bold(),
        package,
        pm.to_string().blue().bold(),
        comment.map_or(String::new(), |c| format!(" with comment: {}", c))
    );

    Ok(())
}

/// remove package from the toml file
pub fn remove_package(pm: &str, package: &str) -> io::Result<()> {
    let base_path = expand_tilde("~/dotfiles/scripts");
    let config_path = Path::new(&base_path).join("omni.toml");

    if !config_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "omni.toml not found. Run init first.",
        ));
    }

    // Read and parse TOML
    let content = fs::read_to_string(&config_path)?;
    let mut doc = content
        .parse::<DocumentMut>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Ensure section exists
    if !doc.as_table().contains_key(pm) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Section [{}] not found in omni.toml", pm),
        ));
    }

    let table = doc[pm]
        .as_table_mut()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid section type"))?;

    // Check if package exists
    if !table.contains_key(package) {
        println!(
            "{} '{}' not found in [{}]",
            "[!]".yellow().bold(),
            package,
            pm.to_string().blue().bold()
        );
        return Ok(());
    }

    // Remove package
    table.remove(package);

    // Write updated TOML back
    fs::write(&config_path, doc.to_string())?;

    println!(
        "{} '{}' removed from [{}]",
        "[-]".red().bold(),
        package,
        pm.to_string().blue().bold()
    );

    Ok(())
}
