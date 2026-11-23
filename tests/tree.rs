use scaffold_cli::tree;

#[test]
fn renders_dirs_before_files_with_arms() {
    let paths = vec![
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "README.md".to_string(),
    ];
    let out = tree::render(&paths);
    assert_eq!(out, "├── src/\n│   ├── lib.rs\n│   └── main.rs\n└── README.md\n");
}

#[test]
fn the_binary_prints_a_tree() {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_scaffold"))
        .args(["describe", "rust", "--tree"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(stdout.contains("src/"));
    assert!(stdout.contains("main.rs"));
    assert!(stdout.contains('├') || stdout.contains('└'));
}
