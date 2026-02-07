use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;
use predicates::str::contains;

// runs omni with no args
#[test]
fn test_cli_no_args() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.assert()
        .success()
        .stdout(contains("OMNI — One Manager"))
        .stdout(contains(env!("CARGO_PKG_NAME")))
        .stdout(contains(env!("CARGO_PKG_VERSION")))
        .stdout(contains(env!("CARGO_PKG_AUTHORS")));
}

// runs omni info
#[test]
fn test_cli_info() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.arg("info");

    cmd.assert()
        .success()
        .stdout(contains("basic info"))
        .stdout(contains(env!("CARGO_PKG_NAME")))
        .stdout(contains(env!("CARGO_PKG_VERSION")))
        .stdout(contains(env!("CARGO_PKG_AUTHORS")));
}

// runs omni install example-package (default)
#[test]
fn test_cli_install_default() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.args(["install", "example-package"]);

    cmd.assert()
        .success()
        .stdout(contains("[DEFAULT] Installing package"))
        .stdout(contains("example-package"));
}

// runs omni install example-package --global
#[test]
fn test_cli_install_global() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.args(["install", "example-package", "--global"]);

    cmd.assert()
        .success()
        .stdout(contains("[GLOBAL] Installing package"))
        .stdout(contains("example-package"));
}

// runs omni install example-package --general
#[test]
fn test_cli_install_general() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.args(["install", "example-package", "--general"]);

    cmd.assert()
        .success()
        .stdout(contains("[GENERAL] Installing package"))
        .stdout(contains("example-package"));
}

// conflicting flags --global and --general
#[test]
fn test_cli_install_conflicting_flags() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.args(["install", "example-package", "--global", "--general"]);

    cmd.assert()
        .failure()
        .stderr(contains("cannot be used with"));
}

// missing required package argument
#[test]
fn test_cli_install_missing_package() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.arg("install");

    cmd.assert().failure().stderr(contains("required"));
}

// unknown subcommand
#[test]
fn test_cli_unknown_subcommand() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.arg("nope");

    cmd.assert()
        .failure()
        .stderr(contains("unrecognized subcommand"));
}

// --help output
#[test]
fn test_cli_help() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(contains("One Manager, No matter the Infrastructure"))
        .stdout(contains("install"))
        .stdout(contains("info"));
}

// --version output
#[test]
fn test_cli_version() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}
