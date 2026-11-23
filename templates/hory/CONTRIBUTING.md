# Contributing

- Commits follow Conventional Commits: `feat:`, `fix:`, `docs:`, `chore:`, `ci:`, `refactor:`, `test:`, `style:`.
- Every commit builds: `cargo check` must pass.
- Before opening a PR: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- Record user-facing changes in [CHANGELOG.md](CHANGELOG.md), Hory-style.
