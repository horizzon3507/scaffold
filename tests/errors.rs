use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_scaffold"))
}

fn run(args: &[&str]) -> std::process::Output {
    bin().args(args).output().unwrap()
}

#[test]
fn unknown_preset_names_the_available_ones() {
    let out = run(&["new", "demo", "-p", "nope", "--dry-run"]);
    assert!(!out.status.success());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("unknown preset 'nope'"));
    assert!(stderr.contains("hory"));
}

#[test]
fn existing_files_require_force() {
    let dir = std::env::temp_dir().join(format!("scaffold-clash-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out = bin()
        .args(["new", "demo", "-p", "rust", "--vcs", "none", "--dir"])
        .arg(&dir)
        .output()
        .unwrap();
    assert!(out.status.success());
    let again = bin()
        .args(["new", "demo", "-p", "rust", "--vcs", "none", "--dir"])
        .arg(&dir)
        .output()
        .unwrap();
    assert!(!again.status.success());
    let stderr = String::from_utf8(again.stderr).unwrap();
    assert!(stderr.contains("--force"));
    let forced = bin()
        .args([
            "new", "demo", "-p", "rust", "--vcs", "none", "--force", "--dir",
        ])
        .arg(&dir)
        .output()
        .unwrap();
    assert!(forced.status.success());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn bad_names_are_rejected() {
    for name in ["", "..", "a/b", "a:b"] {
        let out = run(&["new", name, "--dry-run", "--vcs", "none"]);
        assert!(!out.status.success(), "name {name:?} should fail");
    }
}

#[test]
fn unknown_commands_and_flags_fail_loudly() {
    let out = run(&["nope"]);
    assert!(!out.status.success());
    let out = run(&["list", "--wat"]);
    assert!(!out.status.success());
    let out = run(&["new"]);
    assert!(!out.status.success());
}
