use scaffold_cli::json;

#[test]
fn escapes_quotes_backslashes_and_controls() {
    assert_eq!(json::escape("a\"b\\c\nd"), "a\\\"b\\\\c\\nd");
    assert_eq!(json::string("x"), "\"x\"");
    assert_eq!(json::array(&["1".into(), "2".into()]), "[1,2]");
}

#[test]
fn the_binary_emits_parseable_json() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_scaffold"))
        .args(["list", "--json"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.starts_with("{\"presets\":["));
    assert!(stdout.contains("\"name\":\"hory\""));
    assert!(stdout.trim_end().ends_with('}'));

    let out = std::process::Command::new(env!("CARGO_BIN_EXE_scaffold"))
        .args(["describe", "rust", "--json"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.starts_with("{\"preset\":\"rust\""));
    assert!(stdout.contains("\"path\":\"Cargo.toml\""));
}
