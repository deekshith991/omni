// src/install.rs

use crate::InstallArgs;
use crate::pm::*;
use colorize::*;

pub fn dispatch(args: InstallArgs) {
    if args.global {
        install_global(args.package);
    } else if args.general {
        install_general(args.package);
    } else {
        install_default(args.package);
    }
}

fn install_global(package: String) {
    // Check installed
    if is_installed(&package) {
        println!("Package already installed: {}", package.green());
        return;
    }

    println!("\n[GLOBAL] Installing package: {}", package.green().bold());

    // Detect package manager
    let pm = detect_package_manager().expect("No supported package manager found");

    println!("{}", pm);
}

fn install_general(package: String) {
    // Check installed
    if is_installed(&package) {
        println!("Package already installed: {}", package.green());
        return;
    }

    println!("\n[GENERAL] Installing package: {}", package.green().bold());

    // Detect package manager

    let pm = detect_package_manager().expect("No supported package manager found");
    println!("{}", pm);
}

fn install_default(package: String) {
    // Check installed
    if is_installed(&package) {
        println!("Package already installed: {}", package.green());
        return;
    }

    println!("\n[DEFAULT] Installing package: {}", package.green().bold());

    // Detect package manager
    let pm = detect_package_manager().expect("No supported package manager found");
    println!("{}", pm);
}
