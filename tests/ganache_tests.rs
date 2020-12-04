use assert_cmd::Command;
use std::str::from_utf8;

use serde_json::Value;

#[test]
#[ignore]
fn send_transaction() {
    let send_result = Command::cargo_bin("ruth").unwrap().arg("send").unwrap();

    let tx_hash = from_utf8(&send_result.stdout).unwrap().trim();

    let get_tx_result = Command::cargo_bin("ruth")
        .unwrap()
        .arg("get")
        .arg("tx")
        .arg(tx_hash)
        .unwrap();

    let tx_hash = from_utf8(&get_tx_result.stdout).unwrap().trim();

    let tx: Value = serde_json::from_str(tx_hash).unwrap();

    let from = tx["from"].as_str().unwrap().to_lowercase();

    assert_eq!(from, "0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1");
}
