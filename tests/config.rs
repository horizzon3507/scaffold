use std::process::Command;

use scaffold_cli::config::Config;

#[test]
fn parses_keys_comments_and_quotes() {
    let cfg = Config::parse(
        "# my config\npreset = rust\nauthor = \"hory\"\nvcs = none\nunknown = ignored\nbadline\n",
    );
    assert_eq!(cfg.default_preset.as_deref(), Some("rust"));
    assert_eq!(cfg.author.as_deref(), Some("hory"));
    assert_eq!(cfg.vcs.as_deref(), Some("none"));
}

#[test]
fn empty_and_missing_give_defaults() {
    let cfg = Config::parse("");
    assert!(cfg.default_preset.is_none());
    assert!(cfg.author.is_none());
    assert!(cfg.vcs.is_none());
}

#[test]
fn the_binary_uses_the_config_for_defaults() {
    let dir = std::env::temp_dir().join(format!("scaffold-cfg-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("config");
    std::fs::write(&file, "preset = rust\n").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_scaffold"))
        .args(["new", "cfgdemo", "--dry-run", "--vcs", "none"])
        .env("SCAFFOLD_CONFIG", &file)
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(
        stdout.contains("Cargo.toml"),
        "config preset rust should plan a Cargo.toml:\n{stdout}"
    );
    std::fs::remove_dir_all(&dir).ok();
}
