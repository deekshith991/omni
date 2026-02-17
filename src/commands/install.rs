// src/commands/install.rs

use crate::package_manager;
use colorize::AnsiColor;
use std::io;
use std::process::Command;

/// Main entry for the install command
/// Determines the installation mode and calls the appropriate handler
pub fn run(package: String, general: bool, global: bool, comment: Option<String>) {
    println!(
        "{} Installing package: {}",
        "[+]".green(),
        package.to_string().green().bold()
    );

    if general {
        install_general(&package, comment.as_deref());
    } else if global {
        install_global(&package, comment.as_deref());
    } else {
        install_default(&package, comment.as_deref());
    }
}

/// ------------------------
/// INSTALLATION MODES
/// ------------------------
/// General/local installation
fn install_general(package: &str, comment: Option<&str>) {
    println!("Installing {} locally...", package);

    // Add to [general] section in config
    add_package_to_section("general", package, comment);

    // Attempt system installation if package manager is detected
    if let Ok(pm) = package_manager::detect_package_manager() {
        install_system_package(pm, package);
    }
}

/// Global installation (system-wide)
fn install_global(package: &str, comment: Option<&str>) {
    println!("Installing {} globally...", package);

    match package_manager::detect_package_manager() {
        Ok(pm) => {
            // Add to specific package manager section
            let section = pm_section(&pm);
            add_package_to_section(section, package, comment);

            // Also add to [general] section
            add_package_to_section("general", package, comment);

            // Install via system package manager
            install_system_package(pm, package);
        }
        Err(e) => eprintln!("Error detecting package manager: {}", e),
    }
}

/// Default installation
fn install_default(package: &str, comment: Option<&str>) {
    println!("Installing {} with default mode...", package);

    match package_manager::detect_package_manager() {
        Ok(pm) => {
            // Add to package manager section
            add_package_to_section(pm_section(&pm), package, comment);

            // Install via system package manager
            install_system_package(pm, package);
        }
        Err(e) => eprintln!("Error detecting package manager: {}", e),
    }
}

/// ------------------------
/// HELPER FUNCTIONS
/// ------------------------
/// Add a package to a given section with an optional comment
fn add_package_to_section(section: &str, package: &str, comment: Option<&str>) {
    if let Err(e) = package_manager::add_package(section, package, comment) {
        eprintln!("Failed to add package to [{}]: {}", section, e);
    }
}

/// Map PackageManager enum to string & section name
fn pm_section(pm: &package_manager::PackageManager) -> &str {
    match pm {
        package_manager::PackageManager::Apt => "apt",
        package_manager::PackageManager::Dnf => "dnf",
        package_manager::PackageManager::Pacman => "pacman",
    }
}

/// Install a package using the detected system package manager
fn install_system_package(pm: package_manager::PackageManager, package: &str) {
    let pm_name = pm_section(&pm);

    if let Err(e) = install_package(pm_name, package) {
        eprintln!("System installation failed: {}", e);
    }
}

/// Execute system command to install package
fn install_package(pm: &str, package_name: &str) -> io::Result<()> {
    // Build installation command based on package manager
    let mut cmd = match pm {
        "pacman" => Command::new("sudo")
            .args(["pacman", "-S", package_name, "--noconfirm"])
            .spawn()?,
        "dnf" => Command::new("sudo")
            .args(["dnf", "install", "-y", package_name])
            .spawn()?,
        "apt" => Command::new("sudo")
            .args(["apt", "install", "-y", package_name])
            .spawn()?,
        _ => {
            eprintln!("Unsupported package manager: {}", pm);
            return Ok(());
        }
    };

    // Wait for command to finish
    let status = cmd.wait()?;
    if status.success() {
        println!(
            "{} {} installed successfully using {}!",
            "[+]".green(),
            package_name.to_string().green().bold(),
            pm
        );
    } else {
        eprintln!("Failed to install {} using {}", package_name, pm);
    }

    Ok(())
}
