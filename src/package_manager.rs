// src/package_manager.rs

use which::which;

// Default package_mangers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Pacman,
}

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
