use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_scaffold"))
}

#[test]
fn quiet_suppresses_listing_but_not_errors() {
    let out = bin().args(["list", "--quiet"]).output().unwrap();
    assert!(out.status.success());
    assert!(out.stdout.is_empty());

    let out = bin().args(["new", "demo", "-p", "nope", "--quiet"]).output().unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("unknown preset 'nope'"));
}

#[test]
fn describe_and_new_respect_quiet() {
    let out = bin().args(["describe", "rust", "--quiet"]).output().unwrap();
    assert!(out.status.success());
    assert!(out.stdout.is_empty());

    let dir = std::env::temp_dir().join(format!("scaffold-quiet-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out = bin()
        .args(["new", "demo", "-p", "rust", "--vcs", "none", "--quiet", "--dir"])
        .arg(&dir)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(out.stdout.is_empty());
    assert!(dir.join("Cargo.toml").exists());
    std::fs::remove_dir_all(&dir).ok();
}
