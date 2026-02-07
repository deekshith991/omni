// src/install.rs

use crate::InstallArgs;
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

// Need to try individual method

fn install_global(package: String) {
    println!("\n[GLOBAL] Installing package: {}", package.green().bold());
}

fn install_general(package: String) {
    println!("\n[GENERAL] Installing package: {}", package.green().bold());
}

fn install_default(package: String) {
    println!("\n[DEFAULT] Installing package: {}", package.green().bold());
}
