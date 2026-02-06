// src/install.rs

use colorize::*;

pub fn install_package(package: &str) {
    println!(
        "\nInstalling package: {}",
        format!("{}", package).green().bold()
    );
}
