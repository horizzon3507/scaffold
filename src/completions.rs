//! Hand-written shell completion scripts.

/// Supported shells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Shell {
    pub fn parse(name: &str) -> Option<Shell> {
        match name {
            "bash" => Some(Shell::Bash),
            "zsh" => Some(Shell::Zsh),
            "fish" => Some(Shell::Fish),
            _ => None,
        }
    }
}

/// Preset names completed by every script, from the registry.
fn preset_words() -> String {
    crate::preset::all()
        .iter()
        .map(|p| p.name)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render the completion script for one shell.
pub fn script(shell: Shell) -> String {
    let presets = preset_words();
    match shell {
        Shell::Bash => format!(
            r#"# bash completion for scaffold
_scaffold() {{
    local cur
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    if [ "$COMP_CWORD" -eq 1 ]; then
        COMPREPLY=( $(compgen -W "{presets} completions --verbose --dry-run --quiet" -- "$cur") )
    fi
}}
complete -F _scaffold scaffold
"#
        ),
        Shell::Zsh => format!(
            r#"#compdef scaffold
# zsh completion for scaffold
_scaffold() {{
    local -a items
    items=({presets} completions --verbose --dry-run --quiet)
    _describe 'scaffold' items
}}
_scaffold "$@"
"#
        ),
        Shell::Fish => format!(
            r#"# fish completion for scaffold
for p in {presets}
    complete -c scaffold -f -n "__fish_use_subcommand" -a "$p" -d "preset"
end
complete -c scaffold -f -n "__fish_use_subcommand" -a "completions" -d "print a completion script"
complete -c scaffold -s v -l verbose -d "chatty output"
complete -c scaffold -l dry-run -d "plan only"
"#
        ),
    }
}
