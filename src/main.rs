// src/main.rs

use colorize::*;

fn main() {
    println!(
        "{}",
        "\nOMNI — One Manager, No matter the Infrastructure"
            .green()
            .bold()
    );

    basic_data();
}

fn basic_data() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    println!("{}", format!("{name} v{version}").blue());
    println!("Author(s): {}", authors);
}
