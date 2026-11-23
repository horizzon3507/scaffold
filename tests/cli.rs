//! End-to-end tests for the command line.

use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_scaffold"))
}

#[test]
fn rejects_invalid_project_names() {
    let out = bin()
        .args(["new", "2bad", "--preset", "rust", "--vcs", "none", "--dir", "/tmp/scaffold-bad"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("invalid project name"));
}

#[test]
fn rejects_unknown_presets_and_lists_available() {
    let out = bin()
        .args(["new", "demo", "--preset", "nope", "--vcs", "none", "--dir", "/tmp/scaffold-nope"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("unknown preset 'nope'"));
    assert!(stderr.contains("hory"));
}

#[test]
fn list_shows_every_preset() {
    let out = bin().args(["list"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    for preset in ["hory", "rust", "web", "yuzuha", "python", "node", "go", "docs"] {
        assert!(stdout.contains(preset), "list is missing {preset}");
    }
}

#[test]
fn describe_prints_the_file_tree() {
    let out = bin().args(["describe", "hory"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Cargo.toml"));
    assert!(stdout.contains("src/main.rs"));
}
