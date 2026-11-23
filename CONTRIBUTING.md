# Contributing to scaffold

Thanks for helping out. This project keeps a few disciplines that make the
history easy to read and the binary easy to trust. They are not optional.

## The workflow

1. Open an issue or start a discussion for anything bigger than a typo.
2. Branch from `main`. One branch, one idea.
3. Break the work into commits that each make sense alone (see below).
4. Run the gates, then open a pull request.

## Commits

- Every commit compiles: `cargo check --all-targets` passes at each one.
- Subjects follow Conventional Commits: `feat:`, `fix:`, `docs:`,
  `test:`, `refactor:`, `build:`, `chore:`, `style:`.
- Subjects finish the sentence "this commit will ...", lowercase, no
  trailing period. No numbering, no "wip", no spam commits.
- A feature and its tests travel in the same pull request, usually as
  separate commits.

## Gates before a pull request

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All three must pass. CI runs the same set.

## Zero dependencies

scaffold's binary ships with no runtime dependencies, and that is a
feature. Before suggesting a crate, check whether the standard library
already covers it - the argument parser, the JSON writer, and the shell
completions in this repo were all hand-rolled on purpose. A proposal to
add a dependency belongs in an issue first, with the size cost measured.

## Presets

New presets live in `src/presets/` with their files under `templates/`.
Every template must render with the documented variables, and generated
READMEs keep their license section last.

## Changelog

Record user-facing changes in [CHANGELOG.md](CHANGELOG.md) under
"Unreleased", Hory-style: dated sections, plain sentences.
