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
