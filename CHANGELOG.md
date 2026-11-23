# Changelog

We follow [Semantic Versioning](https://semver.org/) and [Keep a Changelog](https://keepachangelog.com/). scaffold is a single CLI surface.

<details>
<summary>To see more about versioning, expand this.</summary>

Every version string starts with `v` (required), e.g. `v0.1.0-stable`.

Here the installable surface is **CLI** (`scaffold`). Other apps swap in their own names the same way — e.g. **GNOME**, **Desktop**, **Web** — whatever you actually ship.

| Part | What you install | Example |
| --- | --- | --- |
| **CLI** | `scaffold` in the terminal | `v0.1.0-stable` |

With one surface there is no `m` in the tag and no per-surface sections — just the version notes.

Each release heading is the version and date (`## v0.1.0-stable · 22/11/2026`); under it, a short summary ends with a plain sentence like: "This version was made for CLI with a stable release channel on 22/11/2026 (v0.1.0-stable)."

### What the channel suffix means

| Suffix | In plain words |
| --- | --- |
| **-alpha** | Very early. Expect missing pieces and lots of bugs. |
| **-beta** | Mostly there, but still rough. Fine to try; not the "official" install. |
| **-stable** | Ready for daily use. This is what we put on GitHub Releases. |

We only call something **stable** when we mean it.

</details>

## Unreleased

- `scaffold new <name>` generates a project from a preset, with `--preset`, `--dir`, `--force`, `--dry-run`, `--vcs` and `--author`; git initializes by default.
- `scaffold list` and `scaffold describe` inspect the available presets.
- Eight presets: `hory` (the default Hory-style Rust CLI), `rust`, `web`, `yuzuha`, `python`, `node`, `go` and `docs`.
- Templates render `{{name}}`, `{{Name}}`, `{{name_snake}}`, `{{author}}`, `{{year}}` and `{{date}}` in contents and in paths.
