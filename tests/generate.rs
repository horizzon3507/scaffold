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
