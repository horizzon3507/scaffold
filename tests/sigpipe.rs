use std::process::Command;

#[test]
fn piping_into_head_does_not_panic() {
    let out = Command::new("sh")
        .arg("-c")
        .arg(format!("{} list | head -1 2>&1", env!("CARGO_BIN_EXE_scaffold")))
        .output()
        .unwrap();
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(!text.contains("panicked"), "unexpected panic:\n{text}");
    assert!(text.contains("hory"));
}
