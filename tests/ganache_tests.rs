use std::str::from_utf8;

use assert_cmd::Command;
use serde_json::Value;
use serial_test::serial;

#[test]
#[ignore]
#[serial]
fn send_transaction() {
    let block_number_result = Command::cargo_bin("ruth")
        .unwrap()
        .arg("get")
        .arg("block")
        .arg("number")
        .unwrap();

    let block_number_before = from_utf8(&block_number_result.stdout)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

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

    let block_number_result = Command::cargo_bin("ruth")
        .unwrap()
        .arg("get")
        .arg("block")
        .arg("number")
        .unwrap();

    let block_number_after = from_utf8(&block_number_result.stdout)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

    assert_eq!(block_number_after - block_number_before, 1);
}

#[test]
#[ignore]
#[serial]
fn send_transaction_from() {
    let send_result = Command::cargo_bin("ruth").unwrap()
        .arg("send")
        .arg("--from")
        .arg("0xFFcf8FDEE72ac11b5c542428B35EEF5769C409f0")
        .unwrap();

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

    assert_eq!(from, "0xffcf8fdee72ac11b5c542428b35eef5769c409f0");
}


#[test]
#[ignore]
#[serial]
fn send_transaction_value() {
    let send_result = Command::cargo_bin("ruth").unwrap()
        .arg("send")
        .arg("--value")
        .arg("101")
        .unwrap();

    let tx_hash = from_utf8(&send_result.stdout).unwrap().trim();

    let get_tx_result = Command::cargo_bin("ruth")
        .unwrap()
        .arg("get")
        .arg("tx")
        .arg(tx_hash)
        .unwrap();

    let tx_hash = from_utf8(&get_tx_result.stdout).unwrap().trim();

    let tx: Value = serde_json::from_str(tx_hash).unwrap();

    let value = tx["value"].as_str().unwrap();

    assert_eq!(value, "0x65");
}
