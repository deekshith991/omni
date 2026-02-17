// src/commands/install.rs

use crate::package_manager;

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
        }
        Err(e) => eprintln!("Error detecting package manager: {}", e),
    }
}
