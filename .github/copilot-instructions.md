# AI Agent Instructions for Dungeon Crawler World

These instructions apply to Copilot agents and chat-based coding flows in this repository.

## Goals

- Keep changes small, focused, and reviewable.
- Preserve data/schema compatibility between JSON content and Rust types.
- Avoid gameplay regressions and lore/data inconsistencies.

## Repository context

- Rust source code is under `src/`.
- Game data JSON is stored in domain folders such as `Items/`, `Skills/`, `Classes_and_Races/`, `Floor/`, `Magic/`, and related content folders.
- JSON schemas live in `Scheme/`.

## Required validation

Before finalizing code changes, run:

1. `cargo build`
2. `cargo test`
3. `cargo clippy --all-features`

If you modify schema or content data, ensure references and field names remain valid and consistent.

## Change rules

- Follow naming conventions in `CONTRIBUTING.md` and standards in `REPO_STANDARDS.md`.
- Do not introduce new dependencies unless necessary.
- Update docs when behavior or contributor workflow changes.
- Keep instructions and workflows in `.github/` aligned with repository tooling.

## Security and safety

- Never commit secrets, tokens, or credentials.
- Prefer least-privilege permissions in workflows.
- Keep generated AI content grounded in existing project structure and files.
