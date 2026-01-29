use assert_cmd::{cargo_bin, Command};
use predicates::prelude::*;

#[test]
fn test_cli_add_and_get() {
    // Start the binary
    let mut cmd = Command::new(cargo_bin!("memoria"));
    // Simulate user input sequence
    let input = "add\ntext\nHello\nmy_key\nget\nmy_key\nexit\n";
    cmd.write_stdin(input).assert().success().stdout(
        predicate::str::contains("Successfully added!")
            .and(predicate::str::contains("Resource: TextMessage(\"Hello\")")),
    );
}

#[test]
fn test_cli_summary() {
    let mut cmd = Command::new(cargo_bin!("memoria"));
    let input = "add\ntext\nGreeting\nkey1\nadd\nsensor\n25.5\nkey2\nadd\nlog\nLog1,Log2\nkey3\nsummary\nexit\n";
    cmd.write_stdin(input).assert().success().stdout(
        predicate::str::contains("Text messages: 1")
            .and(predicate::str::contains("Sensor data: 1"))
            .and(predicate::str::contains("System logs: 1")),
    );
}

#[test]
fn test_cli_delete() {
    let mut cmd = Command::new(cargo_bin!("memoria"));
    let input = "add\ntext\nTo delete\nkey\nget\nkey\ndelete\nkey\nget\nkey\nexit\n";
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Successfully added!")
                .and(predicate::str::contains(
                    "Resource: TextMessage(\"To delete\")",
                ))
                .and(predicate::str::contains("Resource deleted successfully!")),
        )
        .stderr(predicate::str::contains("Error: Resource 'key' not found"));
}

#[test]
fn test_cli_invalid_command() {
    let mut cmd = Command::new(cargo_bin!("memoria"));
    let input = "invalid\nexit\n";
    cmd.write_stdin(input)
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Error: Input error: Invalid command",
        ));
}

#[test]
fn test_cli_duplicate_key() {
    let mut cmd = Command::new(cargo_bin!("memoria"));
    let input = "add\ntext\nFirst\nkey\nadd\ntext\nSecond\nkey\nexit\n";
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Successfully added!"))
        .stderr(predicate::str::contains(
            "Error: Input error: Key 'key' already exists",
        ));
}
