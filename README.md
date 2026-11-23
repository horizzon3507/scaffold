# scaffold

Spin up new projects from opinionated presets, zero dependencies.

## Install

```sh
cargo install --path .
```

## Usage

```sh
scaffold new my-app            # hory bundle by default
scaffold new my-app -p rust    # pick another preset
scaffold new my-app --dry-run  # see the plan first
```

## Examples

```sh
scaffold new botinho -p yuzuha --author horizzon3507
scaffold describe hory
scaffold new api -p go --dir ~/code/api --vcs none
```

## Presets

| Preset | What you get |
| --- | --- |
| `hory` (default) | Hory-style Rust CLI: clap, CI, Hory changelog, readme, license |
| `rust` | Minimal Rust CLI |
| `web` | Static site: html, css, js |
| `yuzuha` | [Yuzuha](https://github.com/horizzon3507/yuzuha) bot for Fluxer (Node 22.13+) |
| `python` | Python CLI with src layout |
| `node` | Node + TypeScript CLI |
| `go` | Go CLI with Makefile |
| `docs` | Docs-only project |
| `latex` | LaTeX paper: main.tex, Makefile |
| `zig` | Zig CLI: build.zig, main |
| `deno` | Deno TypeScript CLI |
| `flask` | Flask web app |

List them with `scaffold list --verbose`; see a file tree with `scaffold describe <preset>`.

## Template variables

Templates can use these slots, in contents and in file paths:

| Slot | Renders to |
| --- | --- |
| `{{name}}` | `my-app` |
| `{{Name}}` | `My App` |
| `{{name_snake}}` | `my_app` |
| `{{author}}` | `--author`, else `git config user.name` |
| `{{year}}` | the current year |
| `{{date}}` | today as DD/MM/YYYY |

## Configuration

An optional config file at `~/.config/scaffold/config` (or
`$XDG_CONFIG_HOME/scaffold/config`, or `$SCAFFOLD_CONFIG`) sets defaults
in `key = value` lines:

```
preset = rust
author = "hory"
vcs = none
```

Flags always win over the config.

## Colors

Output is colored when stdout is a terminal. Pass `--no-color` or set
the standard `NO_COLOR` environment variable to disable it.

## Quiet mode

`--quiet` prints only errors. Useful in scripts that only care whether
the command worked.

## Machine-readable output

`scaffold list --json` and `scaffold describe <preset> --json` print JSON
for scripts and editors.

## Shell completions

Print a completion script for your shell and source it:

```sh
scaffold completions bash >> ~/.bashrc
scaffold completions zsh  > "${fpath[1]}/_scaffold"   # zsh
scaffold completions fish > ~/.config/fish/completions/scaffold.fish
```

## Binary size

The CLI is dependency-free by design: the argument parser is hand-rolled
and release builds are tuned for size (fat LTO, `opt-level = "z"`,
`panic = "abort"`, stripped symbols). Measurements live in
[SIZES.md](SIZES.md).

## Development

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the workflow: Conventional
Commits, every commit builds, and changes recorded in
[CHANGELOG.md](CHANGELOG.md).

## License

Apache-2.0, see [LICENSE](LICENSE).
