# Changelog

We follow [Semantic Versioning](https://semver.org/) and [Keep a Changelog](https://keepachangelog.com/). {{name}} is a single CLI surface.

<details>
<summary>To see more about versioning, expand this.</summary>

Every version string starts with `v` (required), e.g. `v0.1.0-alpha`.

Here the installable surface is **CLI** (`{{name}}`). Other apps swap in their own names the same way — e.g. **GNOME**, **Desktop**, **Web** — whatever you actually ship.

| Part | What you install | Example |
| --- | --- | --- |
| **CLI** | `{{name}}` in the terminal | `v0.1.0-alpha` |

With one surface there is no `m` in the tag and no per-surface sections — just the version notes.

Each release heading is the version and date (`## v0.1.0-alpha · {{date}}`); under it, a short summary ends with a plain sentence like: "This version was made for CLI with an alpha release channel on {{date}} (v0.1.0-alpha)."

### What the channel suffix means

| Suffix | In plain words |
| --- | --- |
| **-alpha** | Very early. Expect missing pieces and lots of bugs. |
| **-beta** | Mostly there, but still rough. Fine to try; not the "official" install. |
| **-stable** | Ready for daily use. This is what we put on GitHub Releases. |

We only call something **stable** when we mean it.

</details>

## v0.1.0-alpha · {{date}}

First alpha. This version was made for CLI with an alpha release channel on {{date}} (v0.1.0-alpha).

- Initial scaffold from the `hory` preset.
