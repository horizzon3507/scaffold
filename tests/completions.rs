use scaffold_cli::completions::{Shell, script};
use scaffold_cli::preset::all;

#[test]
fn scripts_mention_every_preset() {
    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish] {
        let text = script(shell);
        for p in all() {
            assert!(text.contains(p.name), "{shell:?} missing {}", p.name);
        }
    }
}

#[test]
fn only_known_shells_parse() {
    assert_eq!(Shell::parse("bash"), Some(Shell::Bash));
    assert_eq!(Shell::parse("zsh"), Some(Shell::Zsh));
    assert_eq!(Shell::parse("fish"), Some(Shell::Fish));
    assert_eq!(Shell::parse("powershell"), None);
}
