- This is a game that uses Vulkan for the graphics API

- This game is made with Rust

- Cargo is used to build the application

- The game is based on the Dungeon Crawler Carl series by Matt Dinniman

- JSON is used for data storage

- Markdown is used to present textual elements to the user in some scenarios

- A `package.json` file is used to store project info and access certain developer tools

- A TOML file is used for the Cargo configuration

- YML files are used for Github action workflows

- Txt files are used for developer notes

- `.gitignore` and `.gitattributes` are for Git settings

- `.editorconfig` is for editor settings

- `.lock` is used to lock the Cargo configuration

- The game builds into the `target` directory

- Installed node modules are in the `node_modules` directory

## Build Information

- Use `cargo build` to build the project in debug mode
- Use `cargo build --release` to build the project in release mode
- Use `cargo build --features dev-mode` to build with developer features enabled
- The project uses a custom build script (`build.rs`) that:
  - Checks for required build dependencies
  - Sets up Vulkan configuration
  - Handles JSON data resources
  - Configures platform-specific linking
  - Prints build information including target, profile, and OS

## Test Information

- Use `cargo test` to run all tests
- Tests are located in the `tests/` directory
- Current test suite includes:
  - `new_save_tests.rs`: Tests for save game functionality including:
    - Floor one time generation (both real-time and normal ranges)
    - Character stat generation with proper range validation
- Tests require system dependencies (Vulkan, X11 libraries) to link properly
- For CI environments without GUI dependencies, tests may need to be run with `--no-default-features`

## Development Features

- The project has a `dev-mode` feature flag that enables developer tools
- Developer mode includes an in-game console with various commands
- UI previews can be invoked via console commands when in developer mode
