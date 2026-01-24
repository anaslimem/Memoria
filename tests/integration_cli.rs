use assert_cmd::{Command, cargo_bin};
use predicates::prelude::*;

#[test]
fn test_cli_add_and_get() {
    // Start the binary
    let mut cmd = Command::new(cargo_bin!("memoria"));
    // Simulate user input sequence
    let input = "add\ntext\nHello\nmy_key\nget\nmy_key\nexit\n";
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully added!").and(predicate::str::contains("Resource: TextMessage(\"Hello\")")));
}