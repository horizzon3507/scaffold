use std::process::Command;

#[test]
fn doctor_runs_and_reports() {
    let out = Command::new(env!("CARGO_BIN_EXE_scaffold"))
        .arg("doctor")
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("git on PATH"));
    assert!(stdout.contains("config file"));
    assert!(stdout.contains("presets available"));
    let expected = format!("{} presets", scaffold_cli::preset::all().len());
    assert!(stdout.contains(&expected));
}

#[test]
fn doctor_rejects_extra_arguments() {
    let out = Command::new(env!("CARGO_BIN_EXE_scaffold"))
        .args(["doctor", "extra"])
        .output()
        .unwrap();
    assert!(!out.status.success());
}
