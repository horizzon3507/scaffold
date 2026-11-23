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
