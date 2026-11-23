# scaffold

Scaffolds new projects from opinionated presets, Hory-style.

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

## Binary size

The CLI is dependency-light by design: the argument parser is hand-rolled and
release builds are tuned for size (fat LTO, `opt-level = "z"`, `panic = "abort"`,
stripped symbols). Measurements live in [SIZES.md](SIZES.md).

## Development

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

Commits follow Conventional Commits and every commit builds. Record changes in [CHANGELOG.md](CHANGELOG.md), Hory-style.

## Examples

```sh
scaffold new botinho -p yuzuha --author horizzon3507
scaffold describe hory
scaffold new api -p go --dir ~/code/api --vcs none
```

## License

Apache-2.0, see [LICENSE](LICENSE).
