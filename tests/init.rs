use std::process::Command;

#[test]
fn init_generates_into_the_current_directory() {
    let dir = std::env::temp_dir().join(format!("scaffold-init-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_scaffold"))
        .args(["init", "-p", "rust", "--vcs", "none"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dir.join("Cargo.toml").exists());
    let manifest = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    let dir_name = dir.file_name().unwrap().to_str().unwrap().to_string();
    assert!(manifest.contains(&dir_name));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn init_respects_dry_run() {
    let dir = std::env::temp_dir().join(format!("scaffold-init-dry-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_scaffold"))
        .args(["init", "-p", "rust", "--dry-run"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(!dir.join("Cargo.toml").exists());
    std::fs::remove_dir_all(&dir).ok();
}
