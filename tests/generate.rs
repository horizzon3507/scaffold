//! End-to-end tests for `scaffold new`.

use std::path::PathBuf;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_scaffold"))
}

fn workdir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("scaffold-gen-{}-{}", std::process::id(), tag));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn args_for(dir: &std::path::Path) -> Vec<String> {
    vec![
        "new".into(),
        "demo".into(),
        "--preset".into(),
        "rust".into(),
        "--vcs".into(),
        "none".into(),
        "--dir".into(),
        dir.display().to_string(),
    ]
}

#[test]
fn generates_a_project_end_to_end() {
    let dir = workdir("e2e");
    let out = bin().args(args_for(&dir)).output().unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dir.join("Cargo.toml").exists());
    assert!(dir.join("src/main.rs").exists());
    assert!(!dir.join(".git").exists());
}

#[test]
fn refuses_to_overwrite_without_force() {
    let dir = workdir("clash");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "mine").unwrap();
    let out = bin().args(args_for(&dir)).output().unwrap();
    assert!(!out.status.success());
    assert_eq!(std::fs::read_to_string(dir.join("Cargo.toml")).unwrap(), "mine");
}

#[test]
fn overwrites_with_force() {
    let dir = workdir("force");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "mine").unwrap();
    let mut args = args_for(&dir);
    args.push("--force".into());
    let out = bin().args(args).output().unwrap();
    assert!(out.status.success());
    assert!(std::fs::read_to_string(dir.join("Cargo.toml"))
        .unwrap()
        .contains("demo"));
}

#[test]
fn dry_run_writes_nothing() {
    let dir = workdir("dry");
    let mut args = args_for(&dir);
    args.push("--dry-run".into());
    let out = bin().args(args).output().unwrap();
    assert!(out.status.success());
    assert!(!dir.exists());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("would create"));
}

#[test]
fn initializes_git_unless_skipped() {
    let dir = workdir("vcs");
    let out = bin()
        .args(["new", "demo", "--preset", "rust", "--dir"])
        .arg(&dir)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(dir.join(".git").exists());
}
