
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
    cat ~/dotfiles/scripts/omni.toml
    cargo run -- install vim
    cargo run -- install btop --general
    cargo run -- install btop --general  --comment="general commnet"
    cargo run -- install htop --global --comment="global commant"
    cargo run -- install htop --global
    cat ~/dotfiles/scripts/omni.toml
    cargo run -- install vim
