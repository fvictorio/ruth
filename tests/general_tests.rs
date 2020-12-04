use assert_cmd::Command;

#[test]
fn no_command() {
    let mut cmd = Command::cargo_bin("ruth").unwrap();

    cmd.assert().stderr(predicates::str::contains(
        "A set of command line tools for Ethereum developers",
    ));
}
