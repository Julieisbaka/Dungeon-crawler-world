# Dungeon Crawler World - AI Agent Instructions

## Architecture Overview

This is a **Rust-based dungeon crawler game** inspired by the Dungeon Crawler Carl series. The game uses **egui** for immediate-mode UI and **Vulkan** for graphics rendering, with a unique hybrid JSON+Rust data architecture.

## Key Components & Data Flow

### Core Application (`src/main.rs`)
- **Single-threaded egui app** with persistent state in `DungeonCrawlerworld` struct
- **Developer mode** controlled by Cargo feature `dev-mode` (enabled by default in debug)
- **Global save system** via `CURRENT_SAVE` static mutex
- **Settings persistence** to `settings.json` in project root

### Data Architecture Pattern
**Critical**: This game uses a **dual JSON+Rust** pattern:
- Game entities defined in JSON files under themed directories (`Classes_and_Races/`, `Items/`, `Gods_and_divine_related_entities/`, etc.)
- JSON schemas in `Scheme/` directory validate structure (use `Common.json` for shared properties)
- Rust structs in `src/` mirror JSON for type safety (see `src/player.rs`)
- **Link pattern**: JSON `description` field can reference `.md` files (e.g., `"Items/Crown_of_the_sepsis_whore/description.md"`)

### Save System (`src/saves.rs`, `src/new_save.rs`)
- Saves stored as JSON files in project root
- **State management pattern**: Each UI module has dedicated state struct (e.g., `SaveMenuState`, `NewSaveState`)
- **UI navigation**: Uses boolean flags like `back_requested` for parent/child menu communication

### Developer Tools
- **In-game console** (`src/console.rs`) with command system and log integration
- **UI Preview system** for testing UI components in isolation
- **FPS graph** overlay for performance monitoring
- Build system (`build.rs`) handles Vulkan SDK detection and cross-platform linking

## Development Workflows

### Building & Testing
```bash
cargo build                    # Standard build
cargo test                     # Run unit tests (see tests/new_save_tests.rs)
cargo run --no-default-features  # Disable dev-mode for production
```

### Working with Game Content
1. **New entity types**: Create JSON schema in `Scheme/`, add directory under appropriate category
2. **Items/Spells/Skills**: Follow existing pattern - JSON file + optional markdown description
3. **Player data**: Modify `src/player.rs` and `Scheme/player.json` in tandem
4. **UI changes**: Use state structs pattern, add preview support for dev testing

### JSON Data Conventions
- All entities inherit from `Common.json` schema (name, description, icon, comment)
- **Reference pattern**: Use relative paths for cross-references (skills, spells, benefits)
- **Description linking**: JSON description can be string or path to `.md` file
- **Validation**: JSON schemas enforce structure but Rust code handles loading

### Platform-Specific Notes
- **Windows**: Uses WiX for MSI packaging, links user32/gdi32/shell32
- **macOS**: MoltenVK for Vulkan-over-Metal, framework linking in build.rs
- **Linux**: pkg-config for Vulkan detection, X11/XRandr dependencies

## Critical Patterns to Follow

1. **State Management**: Always use dedicated state structs for UI modules
2. **JSON+Rust Sync**: When modifying data structures, update both JSON schema and Rust struct
3. **Developer Mode Gating**: Wrap dev features with `DEV_MODE_ENABLED` const
4. **Error Handling**: Use `Option`/`Result` patterns, avoid panics in UI code
5. **Settings Persistence**: Settings auto-save on modification, load on startup

## File Organization Logic
- `src/` - Pure Rust code (game logic, UI, systems)  
- `Scheme/` - JSON schemas for validation
- Content directories - Game data (Items, Classes, Gods, etc.) as JSON+markdown pairs
- Root level - Settings, saves, build configs
