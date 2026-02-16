// src/commands/install.rs

pub fn run(package: String, local: bool, global: bool) {
    println!("Installing package: {}", package);

    if local {
        install_local(&package);
    } else if global {
        install_global(&package);
    } else {
        install_default(&package);
    }
}

// local install logic
fn install_local(package: &str) {
    println!("Installing {} locally...", package);
}

// global install logic
fn install_global(package: &str) {
    println!("Installing {} globally...", package);
}

// default install logic
fn install_default(package: &str) {
    println!("Installing {} with default mode...", package);
}
