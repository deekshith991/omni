// src/commands/install.rs

use crate::package_manager;
use std::process::Command;
use std::io;

/// Main entry for install command
pub fn run(package: String, general: bool, global: bool, comment: Option<String>) {
    println!("Installing package: {}", package);

    if general {
        install_general(&package, comment);
    } else if global {
        install_global(&package, comment);
    } else {
        install_default(&package, comment);
    }
}

/// General install
fn install_general(package: &str, comment: Option<String>) {
    println!("Installing {} locally...", package);

    // Add to [general] section
    if let Err(e) = package_manager::add_package("general", package, comment.as_deref()) {
        eprintln!("Failed to add package to [general]: {}", e);
    }

    // Optional: Attempt system install if you want
    if let Ok(pm) = package_manager::detect_package_manager() {
        if let Err(e) = install_package(pm_str(&pm), package) {
            eprintln!("System installation failed: {}", e);
        }
    }
}

/// Global install
fn install_global(package: &str, comment: Option<String>) {
    println!("Installing {} globally...", package);

    match package_manager::detect_package_manager() {
        Ok(pm) => {
            let section = match pm {
                package_manager::PackageManager::Apt => "apt",
                package_manager::PackageManager::Dnf => "dnf",
                package_manager::PackageManager::Pacman => "pacman",
            };

            // Add to package manager section
            if let Err(e) = package_manager::add_package(section, package, comment.as_deref()) {
                eprintln!("Failed to add package to [{}]: {}", section, e);
            }

            // Add to [general] section
            if let Err(e) = package_manager::add_package("general", package, comment.as_deref()) {
                eprintln!("Failed to add package to [general]: {}", e);
            }

            // Install system package
            if let Err(e) = install_package(pm_str(&pm), package) {
                eprintln!("System installation failed: {}", e);
            }
        }
        Err(e) => eprintln!("Error detecting package manager: {}", e),
    }
}

/// Default install
fn install_default(package: &str, comment: Option<String>) {
    println!("Installing {} with default mode...", package);

    match package_manager::detect_package_manager() {
        Ok(pm) => {
            let section = match pm {
                package_manager::PackageManager::Apt => "apt",
                package_manager::PackageManager::Dnf => "dnf",
                package_manager::PackageManager::Pacman => "pacman",
            };

            if let Err(e) = package_manager::add_package(section, package, comment.as_deref()) {
                eprintln!("Failed to add package to [{}]: {}", section, e);
            }

            // Install system package
            if let Err(e) = install_package(pm_str(&pm), package) {
                eprintln!("System installation failed: {}", e);
            }
        }
        Err(e) => eprintln!("Error detecting package manager: {}", e),
    }
}

/// Convert PackageManager enum to &str
fn pm_str(pm: &package_manager::PackageManager) -> &str {
    match pm {
        package_manager::PackageManager::Apt => "apt",
        package_manager::PackageManager::Dnf => "dnf",
        package_manager::PackageManager::Pacman => "pacman",
    }
}

/// Install package using system package manager
fn install_package(pm: &str, package_name: &str) -> io::Result<()> {
    // Determine the installation command based on the package manager
    let mut cmd = match pm {
        "pacman" => Command::new("sudo")
            .args(&["pacman", "-S", package_name, "--noconfirm"])
            .spawn()?,
        "dnf" => Command::new("sudo")
            .args(&["dnf", "install", "-y", package_name])
            .spawn()?,
        "apt" => Command::new("sudo")
            .args(&["apt", "install", "-y", package_name])
            .spawn()?,
        _ => {
            eprintln!("Unsupported package manager: {}", pm);
            return Ok(());
        }
    };

    // Wait for the command to finish
    let status = cmd.wait()?;
    if status.success() {
        println!("{} installed successfully using {}!", package_name, pm);
    } else {
        eprintln!("Failed to install {} using {}", package_name, pm);
    }

    Ok(())
}
