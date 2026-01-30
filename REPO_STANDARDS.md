# Repository Standards

This document summarizes formatting, styling, and repository standards for Dungeon Crawler World. It complements [CONTRIBUTING.md](CONTRIBUTING.md) and the architecture notes in [AI agent instructions](.github/copilot-instructions.md).

## Formatting & Styling

- **EditorConfig**: Follow `.editorconfig` (UTF-8, CRLF line endings, 2-space indentation, final newline, trim trailing whitespace except in Markdown).
- **Markdown**: Markdown lint rules are relaxed in `.markdownlint.json`, but keep formatting consistent and readable.
- **Naming conventions** (from CONTRIBUTING):
  - **PascalCase**: Rust types (e.g., structs, enums, traits)
  - **snake_case**: Rust functions/methods, variables, and struct fields
  - **snake_case**: JSON properties
- **Comments**: Keep comments clear and descriptive; avoid unnecessary commentary.

## Data Architecture Standards

- **Dual JSON + Rust pattern**: game entities live in JSON under themed directories and are mirrored by Rust structs in `src/` for type safety.
- **Schema updates required**: update JSON Schema files in `Scheme/` whenever data structures change.
- **Description linking**: JSON `description` fields may link to `.md` files using relative paths.
- **Common schema**: shared entity fields live in `Scheme/Common.json`.

## UI & State Management

- **State structs**: each UI module should have a dedicated state struct (e.g., `SaveMenuState`, `NewSaveState`).
- **Navigation flags**: UI flow uses boolean flags like `back_requested`.
- **Developer mode gating**: wrap dev-only features with `DEV_MODE_ENABLED`.

## Project Organization

- `src/` contains Rust code (game logic, UI, systems).
- `Scheme/` contains JSON schemas.
- Content directories (`Items/`, `Skills/`, `Classes_and_Races/`, etc.) contain JSON data and optional markdown descriptions.
- Root-level files store settings and save data as JSON.

## Build & Test Expectations

- **Build**: `cargo build`
- **Tests**: `cargo test`
- **Production build**: `cargo run --no-default-features`

## Security & Conduct

- Follow the [Code of Conduct](CODE_OF_CONDUCT.md).
- Report security issues via [SECURITY.md](SECURITY.md).
