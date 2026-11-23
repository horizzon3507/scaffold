//! Generated trees from the presets.

use std::path::PathBuf;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_scaffold"))
}

fn generate(preset: &str, tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("scaffold-preset-{}-{}", std::process::id(), tag));
    let _ = std::fs::remove_dir_all(&dir);
    let out = bin()
        .args([
            "new", "demo-app", "--preset", preset, "--vcs", "none", "--author", "tester",
            "--dir",
        ])
        .arg(&dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    dir
}

fn assert_files(dir: &std::path::Path, expected: &[&str]) {
    for path in expected {
        assert!(dir.join(path).exists(), "missing {path}");
    }
}

#[test]
fn hory_generates_a_rust_cli() {
    let dir = generate("hory", "hory");
    assert_files(
        &dir,
        &[
            "Cargo.toml",
            "src/main.rs",
            "src/args.rs",
            "CHANGELOG.md",
            "README.md",
            "LICENSE",
            ".gitignore",
            ".editorconfig",
            "rustfmt.toml",
            "CONTRIBUTING.md",
            ".github/workflows/ci.yml",
            ".github/ISSUE_TEMPLATE/bug_report.md",
        ],
    );
    let manifest = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("demo-app"));
    assert!(manifest.contains("tester"));
    let changelog = std::fs::read_to_string(dir.join("CHANGELOG.md")).unwrap();
    assert!(!changelog.contains("{{"));
}

#[test]
fn rust_generates_a_minimal_cli() {
    let dir = generate("rust", "rust");
    assert_files(&dir, &["Cargo.toml", "src/main.rs", "README.md", "CHANGELOG.md", "LICENSE", ".gitignore"]);
    let manifest = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("demo-app"));
}

#[test]
fn web_generates_a_static_site() {
    let dir = generate("web", "web");
    assert_files(&dir, &["index.html", "style.css", "app.js", "package.json", "README.md", ".gitignore"]);
    let html = std::fs::read_to_string(dir.join("index.html")).unwrap();
    assert!(html.contains("Demo App"));
}

#[test]
fn yuzuha_generates_a_bot() {
    let dir = generate("yuzuha", "yuzuha");
    assert_files(&dir, &["package.json", "tsconfig.json", "commands/ping.ts", "commands/eco.ts", ".env.example", ".gitignore", "README.md"]);
    let pkg = std::fs::read_to_string(dir.join("package.json")).unwrap();
    assert!(pkg.contains("yuzuha"));
}

#[test]
fn python_generates_a_package() {
    let dir = generate("python", "python");
    assert_files(&dir, &["pyproject.toml", "src/demo_app/__init__.py", "src/demo_app/__main__.py", "tests/test_smoke.py", "README.md", ".gitignore", "LICENSE"]);
    let pyproject = std::fs::read_to_string(dir.join("pyproject.toml")).unwrap();
    assert!(pyproject.contains("demo_app"));
}
