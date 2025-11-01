use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn bin_cmd() -> Command {
    let bin_path = assert_cmd::cargo::cargo_bin!("nanji");
    Command::new(bin_path)
}

#[test]
fn default_includes_common_timezones() {
    let mut cmd = bin_cmd();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Asia/Tokyo"))
        .stdout(predicate::str::contains("America/Chicago"));
}

#[test]
fn base_tokyo_reflects_input_minutes() {
    let mut cmd = bin_cmd();
    cmd.args(["-b", "tokyo", "-t", "09:10"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Asia/Tokyo"))
        .stdout(predicate::str::contains("09:10"));
}

#[test]
fn base_dallas_reflects_input_minutes() {
    let mut cmd = bin_cmd();
    cmd.args(["-b", "dallas", "-t", "09:00"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("America/Chicago"))
        .stdout(predicate::str::contains("09:00"));
}

#[test]
fn zones_filter_limits_output() {
    let mut cmd = bin_cmd();
    cmd.arg("--zones").arg("Asia/Tokyo,America/Chicago");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Asia/Tokyo"))
        .stdout(predicate::str::contains("America/Chicago"))
        .stdout(predicate::str::contains("America/Los_Angeles").not());
}

#[test]
fn invalid_time_in_base_mode_shows_error() {
    let mut cmd = bin_cmd();
    cmd.args(["-b", "tokyo", "-t", "31:00"]);
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("invalid time format"));
}

#[test]
fn alias_labels_output_when_requested() {
    let mut cmd = bin_cmd();
    cmd.arg("--alias").arg("--zones").arg("tokyo,dallas");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tokyo"))
        .stdout(predicate::str::contains("dallas"));
}

#[test]
fn base_without_time_is_error() {
    let mut cmd = bin_cmd();
    cmd.args(["-b", "tokyo"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--time"));
}

#[test]
fn time_without_base_is_error() {
    let mut cmd = bin_cmd();
    cmd.args(["-t", "09:00"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--base"));
}
