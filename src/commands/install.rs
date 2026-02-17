// src/commands/install.rs

pub fn run(package: String, local: bool, global: bool) {
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
    println!("optional comment {}", comment.unwrap_or_default());
    println!("Installing {} locally...", package);
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
