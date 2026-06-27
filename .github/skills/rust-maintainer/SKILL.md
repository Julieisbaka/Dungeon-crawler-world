---
name: rust-maintainer
description: >-
  Skill for making safe Rust gameplay/system changes in this repository while preserving
  schema/data compatibility and running the project's required validation commands.
user-invocable: true
---

# Rust Maintainer Skill

Use this skill for Rust implementation and refactoring tasks in `src/`.

## Expectations

- Keep patches focused and minimal.
- Match repository naming and style conventions.
- Avoid introducing unnecessary dependencies.

## Validation checklist

Run these commands before finalizing:

1. `cargo build`
2. `cargo test`
3. `cargo clippy --all-features`

## Data compatibility reminders

- If a Rust type mirrors JSON data, confirm compatibility with files in content directories.
- If structure changes are required, update corresponding files under `Scheme/`.
