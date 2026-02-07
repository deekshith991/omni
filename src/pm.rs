use std::process::Command;

pub fn detect_package_manager() -> Option<String> {
    if Command::new("which").arg("apt").output().is_ok() {
        return Some("apt".into());
    }
    if Command::new("which").arg("dnf").output().is_ok() {
        return Some("dnf".into());
    }
    if Command::new("which").arg("pacman").output().is_ok() {
        return Some("pacman".into());
    }
    None
}

pub fn is_installed(pkg: &str) -> bool {
    println!("checking for installation");

    if Command::new("dpkg-query")
        .arg("-W")
        .arg(pkg)
        .output()
        .map_or(false, |o| o.status.success())
    {
        return true;
    }

    if Command::new("rpm")
        .arg("-q")
        .arg(pkg)
        .output()
        .map_or(false, |o| o.status.success())
    {
        return true;
    }

    if Command::new("pacman")
        .arg("-Qs")
        .arg(pkg)
        .output()
        .map_or(false, |o| o.status.success())
    {
        return true;
    }

    false
}

pub fn install_with_pm(pm: &str, pkg: &str) {
    let output = match pm {
        "apt" => Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg(pkg)
            .output(),
        "dnf" => Command::new("sudo")
            .arg("dnf")
            .arg("install")
            .arg("-y")
            .arg(pkg)
            .output(),
        "pacman" => Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg(pkg)
            .output(),
        _ => panic!("Unsupported package manager"),
    }
    .expect("Failed to run package manager");

    if !output.status.success() {
        panic!("Package install failed");
    }
}
