
default:
    @echo "cleaning the project"
    cargo fmt
    cargo clean

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
    cargo run -- install vim
    cargo run -- install vim --global
    cargo run -- install vim --local