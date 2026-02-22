// src/commands/remove.rs

use crate::package_manager;
use colorize::AnsiColor;
use std::io;
use std::process::Command;

/// Main entry for the remove command
pub fn run(package: String, all: bool, pm: bool, general: bool, global: bool) {
    println!(
        "{} Removing package: {}",
        "[-]".red(),
        package.to_string().red().bold()
    );

    if all {
        remove_all(&package);
    } else if pm {
        remove_pm_only(&package);
    } else if general {
        remove_general(&package);
    } else if global {
        remove_global(&package);
    } else {
        remove_default(&package);
    }
}

/// ------------------------
/// REMOVAL MODES
/// ------------------------
/// Remove from everywhere (general + pm section + uninstall)
fn remove_all(package: &str) {
    println!("Removing {} from all sections...", package);

    if let Ok(pm_detected) = package_manager::detect_package_manager() {
        let section = pm_section(&pm_detected);

        remove_from_section("general", package);
        remove_from_section(section, package);

        remove_system_package(pm_detected, package);
    }
}

/// Remove only from package manager section
fn remove_pm_only(package: &str) {
    println!("Removing {} from package manager section...", package);

    match package_manager::detect_package_manager() {
        Ok(pm_detected) => {
            let section = pm_section(&pm_detected);
            remove_from_section(section, package);
        }
        Err(e) => eprintln!("Error detecting package manager: {}", e),
    }
}

/// Remove only from [general]
fn remove_general(package: &str) {
    println!("Removing {} from [general]...", package);
    remove_from_section("general", package);
}

/// Remove globally (pm section + general + uninstall)
fn remove_global(package: &str) {
    println!("Removing {} globally...", package);

    match package_manager::detect_package_manager() {
        Ok(pm_detected) => {
            let section = pm_section(&pm_detected);

            remove_from_section(section, package);
            remove_from_section("general", package);

            remove_system_package(pm_detected, package);
        }
        Err(e) => eprintln!("Error detecting package manager: {}", e),
    }
}

/// Default removal (pm section + uninstall only)
fn remove_default(package: &str) {
    println!("Removing {} with default mode...", package);

    match package_manager::detect_package_manager() {
        Ok(pm_detected) => {
            remove_from_section(pm_section(&pm_detected), package);
            remove_system_package(pm_detected, package);
        }
        Err(e) => eprintln!("Error detecting package manager: {}", e),
    }
}

/// ------------------------
/// HELPER FUNCTIONS
/// ------------------------
fn remove_from_section(section: &str, package: &str) {
    if let Err(e) = package_manager::remove_package(section, package) {
        eprintln!("Failed to remove package from [{}]: {}", section, e);
    }
}

fn pm_section(pm: &package_manager::PackageManager) -> &str {
    match pm {
        package_manager::PackageManager::Apt => "apt",
        package_manager::PackageManager::Dnf => "dnf",
        package_manager::PackageManager::Pacman => "pacman",
    }
}

fn remove_system_package(pm: package_manager::PackageManager, package: &str) {
    let pm_name = pm_section(&pm);

    if let Err(e) = uninstall_package(pm_name, package) {
        eprintln!("System removal failed: {}", e);
    }
}

/// Execute system command to uninstall package
fn uninstall_package(pm: &str, package_name: &str) -> io::Result<()> {
    let mut cmd = match pm {
        "pacman" => Command::new("sudo")
            .args(["pacman", "-Rns", package_name, "--noconfirm"])
            .spawn()?,
        "dnf" => Command::new("sudo")
            .args(["dnf", "remove", "-y", package_name])
            .spawn()?,
        "apt" => Command::new("sudo")
            .args(["apt", "remove", "-y", package_name])
            .spawn()?,
        _ => {
            eprintln!("Unsupported package manager: {}", pm);
            return Ok(());
        }
    };

    let status = cmd.wait()?;

    if status.success() {
        println!(
            "{} {} removed successfully using {}!",
            "[-]".red(),
            package_name.to_string().red().bold(),
            pm
        );
    } else {
        eprintln!("Failed to remove {} using {}", package_name, pm);
    }

    Ok(())
}