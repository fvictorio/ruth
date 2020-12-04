use assert_cmd::Command;
use std::str::from_utf8;

#[test]
fn random_address() {
    let mut cmd = Command::cargo_bin("ruth").unwrap();

    cmd.arg("random")
        .arg("address")
        .assert()
        .stdout(predicates::str::starts_with("0x"));
}

#[test]
fn successive_random_address_different() {
    let result1 = Command::cargo_bin("ruth")
        .unwrap()
        .arg("random")
        .arg("address")
        .unwrap();

    let result2 = Command::cargo_bin("ruth")
        .unwrap()
        .arg("random")
        .arg("address")
        .unwrap();

    assert_ne!(result1, result2);
}

#[test]
fn count_2_random_address_different() {
    let result = Command::cargo_bin("ruth")
        .unwrap()
        .arg("random")
        .arg("address")
        .arg("--count")
        .arg("2")
        .unwrap();

    let output = from_utf8(&result.stdout).unwrap();

    let addresses: Vec<_> = output.trim().split("\n").collect();

    assert_eq!(addresses.len(), 2);

    assert_ne!(addresses[0], addresses[1]);
}

#[test]
fn random_bytes() {
    Command::cargo_bin("ruth")
        .unwrap()
        .arg("random")
        .arg("bytes")
        .assert()
        .stdout(predicates::str::starts_with("0x"));
}

#[test]
fn random_bytes_with_length() {
    let result = Command::cargo_bin("ruth")
        .unwrap()
        .arg("random")
        .arg("bytes")
        .arg("--length")
        .arg("4")
        .unwrap();

    let stdout = from_utf8(&result.stdout).unwrap().trim();

    assert_eq!(stdout.len(), 10);
}

