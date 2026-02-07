// tests/cli.rs

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;
use predicates::str::contains;

// runs omni
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

// runs omni install example-package
#[test]
fn test_cli_install() {
    let mut cmd = cargo_bin_cmd!("omni");
    cmd.arg("install").arg("example-package");
    cmd.assert().success().stdout(contains("example-package"));
}
