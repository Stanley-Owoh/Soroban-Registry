use std::env;
use std::path::PathBuf;
use std::process::Command;

fn get_binary_path() -> PathBuf {
    let name_hyphen = "soroban-registry";
    let name_underscore = "soroban_registry";

    if let Ok(path) = env::var(format!("CARGO_BIN_EXE_{}", name_underscore)) {
        return PathBuf::from(path);
    }
    if let Ok(path) = env::var(format!("CARGO_BIN_EXE_{}", name_hyphen)) {
        return PathBuf::from(path);
    }

    let mut path = env::current_dir().expect("Failed to get current dir");
    path.push("target");
    path.push("debug");
    path.push(name_hyphen);
    if path.exists() {
        return path;
    }
    path.set_extension("exe");
    if path.exists() {
        return path;
    }

    panic!("Could not find binary path via env var. Ensure `cargo build` has run.");
}

#[test]
fn test_search_help() {
    let output = Command::new(get_binary_path())
        .arg("search")
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--verified-only"));
    assert!(stdout.contains("--category"));
    assert!(stdout.contains("--limit"));
    assert!(stdout.contains("--offset"));
    assert!(stdout.contains("--json"));
    assert!(stdout.contains("--network"));
}

#[test]
fn test_search_fails_gracefully_without_api() {
    let output = Command::new(get_binary_path())
        .arg("--api-url")
        .arg("http://127.0.0.1:9999") // Use a port that is unlikely to be in use
        .arg("search")
        .arg("token")
        .output()
        .expect("Failed to execute command");

    // The command should fail because it can't connect to the API.
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Check for the expected error message from the `search` function's context.
    assert!(stderr.contains("Failed to search contracts"));
    // Ensure it's not an argument parsing error.
    assert!(!stderr.contains("unexpected argument"));
}

#[test]
fn test_search_with_all_flags_parses_correctly() {
    let output = Command::new(get_binary_path())
        .arg("--api-url")
        .arg("http://127.0.0.1:9999")
        .arg("search")
        .arg("test-query")
        .arg("--verified-only")
        .arg("--category")
        .arg("defi")
        .arg("--limit")
        .arg("5")
        .arg("--offset")
        .arg("10")
        .arg("--network")
        .arg("testnet")
        .arg("--json")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to search contracts"));
    assert!(!stderr.contains("unexpected argument"));
}
