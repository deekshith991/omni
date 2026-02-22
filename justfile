
default:
    @echo "cleaning the project"
    cargo fmt
    cargo clean

clean:
    @echo "cleaning the project"
    cargo fmt
    cargo clean
    sudo apt remove vim htop btop
    rm ~/dotfiles/scripts/omni.toml

build:
    @echo "building project"
    cargo fmt
    cargo build

release:
    @echo "releasing project"
    cargo fmt
    cargo build --release

run-cmd:
    cargo run -- init
    cargo run -- list
    cargo run -- install vim
    cargo run -- install btop --general
    cargo run -- install btop --general  --comment="general commnet"
    cargo run -- install htop --global --comment="global commant"
    cargo run -- install htop --global
    cargo run -- list

push:
    @echo "Formatting the code..."
    cargo fmt
    @echo "Checking code with clippy..."
    cargo clippy -- -D warnings
    @echo "No clippy warnings! Pushing to git..."
    git push

publish tag:
    @echo "Formatting the code..."
    cargo fmt
    @echo "Checking code with clippy..."
    cargo clippy -- -D warnings
    @echo "No clippy warnings! Pushing to git..."
    @echo "Creating Git tag {{tag}}..."
    git tag {{tag}}
    git push
    @echo "Pushing tag {{tag}} to remote..."
    git push origin {{tag}}
