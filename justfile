# Default recipe
default:
    just check

# Format and check the project
check:
    cargo fmt
    cargo check

# Build the project and set temporary aliases
build:
    cargo fmt
    cargo build
    @echo "Setting temporary aliases:"
    @echo "alias p='edit ~/dotfiles/scripts/packages.toml'"
    @echo "alias om='./target/debug/omni'"
    @echo "Run this to activate them in your shell:"
    @echo "eval \"\$(just build)\""

# Run in debug mode
run:
    cargo fmt
    cargo run

# Run tests
test:
    cargo fmt
    cargo test

# Build release binary
release:
    cargo fmt
    cargo build --release

# Clean artifacts and remove temporary aliases
clean:
    cargo clean
    @echo "Removing temporary aliases..."
    @unset p
    @unset om
    @echo "Aliases removed."
