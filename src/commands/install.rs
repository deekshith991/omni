// src/commands/install.rs

use crate::package_manager;

pub fn run(package: String, local: bool, global: bool, comment: Option<String>) {
    println!("Installing package: {}", package);

    if local {
        install_local(&package, comment);
    } else if global {
        install_global(&package, comment);
    } else {
        install_default(&package, comment);
    }
}

// local install logic
fn install_local(package: &str, comment: Option<String>) {
    println!("Installing {} locally...", package);
    println!("optional comment {}", comment.unwrap_or_default());


    // getting default package_manager
    match package_manager::detect_package_manager() {
        // Ok(pm) => install_with_manager(pm, package),
        Ok(pm) => println!("Installed {:?} {}", pm, package),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// global install logic
fn install_global(package: &str, comment: Option<String>) {
    println!("Installing {} globally...", package);
    println!("optional comment {}", comment.unwrap_or_default());
}

// default install logic
fn install_default(package: &str, comment: Option<String>) {
    println!("Installing {} with default mode...", package);
    println!("optional comment {}", comment.unwrap_or_default());
}
