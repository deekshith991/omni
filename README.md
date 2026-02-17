# Omni – One Manager, No matter the Infrastructure

**Omni** is a lightweight and versatile package manager designed to simplify the installation and management of software packages on Linux systems. It supports local, global, and default installation modes, integrating with your system package manager when available.

---

## Features

- 💻 **Multi-mode installation**: Install packages locally, globally, or using default settings.
- 🔍 **System package manager integration**: Automatically detects and installs packages via `apt`, `dnf`, or `pacman`.
- 📝 **Optional installation comments**: Document why a package was installed.
- 🛠️ **Easy configuration**: Packages are tracked in relevant sections of your config file.
- 🟢 **Colorized CLI output** for better readability.

---

## Installation

Clone the repository and build the tool using Rust:

```bash
git clone https://github.com/yourusername/omni.git
cd omni
cargo build --release
mv ./target/release/omni /usr/local/bin/
source ~/.bashrc
omni init
```

After building, the binary will be available at:

```bash
./target/release/omni
```

---

## Usage

Omni uses a CLI interface powered by [clap](https://crates.io/crates/clap).

```bash
omni <COMMAND> [OPTIONS]
```

### Commands

#### `init`

Initialize a new Omni package tracking functionality

```bash
omni init
```

#### `install`

Install a package with various options:

```bash
omni install <PACKAGE> [OPTIONS]
```

---

### Installation Modes Table

| Installation Mode      | Command Example                                   | Description                                      | Notes                                      |
|------------------------|--------------------------------------------------|--------------------------------------------------|--------------------------------------------|
| **General / Local**     | `omni install <PACKAGE> --general`             | Installs the package locally for the user/project | Adds to `[general]` section; optional comment with `--comment` |
| **Global / System-wide**| `omni install <PACKAGE> --global`              | Installs the package system-wide (requires sudo) | Adds to `[general]` and package manager section; uses system package manager |
| **Default / Auto**      | `omni install <PACKAGE>`                        | Installs using the detected system package manager | Adds to package manager section; chooses based on `apt`, `dnf`, or `pacman` |
| **With Comment**        | `omni install <PACKAGE> --general --comment "..."` | Adds an installation comment for documentation  | Works with any mode (`--general` or `--global`) |

---

## Supported Package Managers

Omni detects and integrates with the following system package managers:

- **Apt** – Debian/Ubuntu based systems
- **Dnf** – Fedora/RHEL based systems
- **Pacman** – Arch Linux based systems

If no supported package manager is found, Omni will still record the package in your configuration.

---

## Configuration

Packages are organized into sections in the configuration file:

- `[general]` – Local or general packages
- `[apt]`, `[dnf]`, `[pacman]` – Packages installed via specific system package managers

You can optionally add comments to each package for documentation purposes.

---

## Contributing

Contributions are welcome! Feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements

Before contributing, make sure your code passes `cargo fmt` and `cargo clippy`.

---

## License

This project is licensed under the **MIT License**. See `LICENSE` for details.

---

## Acknowledgements

- Built with **Rust** and **clap** for CLI parsing.
- Colorized output via **colorize** crate.  
