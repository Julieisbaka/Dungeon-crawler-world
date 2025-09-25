# Contributing to Dungeon Crawler World

Thank you for your interest in contributing to Dungeon Crawler World! This
document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Building and Testing](#building-and-testing)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Schema Guidelines](#schema-guidelines)
- [Issue Reporting](#issue-reporting)

## Code of Conduct

By participating in this project, you agree to maintain a respectful and
inclusive environment for everyone. Please be considerate of others and
communicate professionally. To view the full Code of Conduct please read the [Code of Conduct](CODE_OF_CONDUCT.md) which is adapted from the Mozilla code of conduct and contributor covenant.

## Getting Started

1. Setup [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
2. Install and setup [Rust](https://www.rust-lang.org/tools/install) (version 1.70 or later recommended)
3. Install system dependencies (platform-specific):
   - **Linux**: `sudo apt-get install libvulkan-dev libx11-dev libxrandr-dev libxcursor-dev libxi-dev`
   - **Windows**: Install the [Vulkan SDK](https://vulkan.lunarg.com/sdk/home)
   - **macOS**: Install the [Vulkan SDK](https://vulkan.lunarg.com/sdk/home) and ensure Xcode command line tools are installed
4. Clone or fork the repository: `git clone https://github.com/Julieisbaka/Dungeon-crawler-world.git`
5. `cd` into the project directory: `cd Dungeon-crawler-world`
6. Build the project: `cargo build`
7. Run the project: `cargo run`

## Development Setup

### Prerequisites

- Rust 1.70+ with Cargo
- Vulkan drivers and SDK
- Platform-specific GUI libraries (X11, etc.)
- Node.js (for development tools and package.json scripts)

### Feature Flags

- `dev-mode` (default): Enables developer tools and console
- `--no-default-features`: Disables developer mode for release builds

### Project Structure

- `src/`: Main source code
  - `main.rs`: Application entry point and main game loop
  - `console.rs`: Developer console implementation
  - `saves.rs`: Save game functionality
  - `settings.rs`: Game settings management
  - `skills.rs`: Character skills system
  - `player.rs`: Player data structures
- `tests/`: Test suites including save game and stat generation tests
- `build.rs`: Custom build script for platform setup
- `Scheme/`: JSON schema files for data validation

## Building and Testing

### Build Commands

```bash
# Development build (with dev-mode features)
cargo build --features dev-mode

# Release build (optimized, no dev features)
cargo build --release --no-default-features

# Run the application
cargo run

# Run with specific features
cargo run --features dev-mode
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test file
cargo test new_save_tests

# Run tests with output
cargo test -- --nocapture
```

**Note**: Tests require system GUI libraries to link properly. In CI environments without GUI support, tests may fail at the linking stage.

### Automated Workflows

The project includes several GitHub Actions workflows:
- **CodeQL**: Security scanning on push/PR to main
- **DevSkim**: Security analysis on push/PR to Main branch  
- **Dependency Review**: Vulnerability scanning on PRs
- **Build and Release**: Manual release building for Windows/macOS
- **Lock File Update**: Automatic Cargo.lock updates

## How to Contribute

There are many ways to contribute to Dungeon Crawler World:

- **Code contributions** - Implement new features or fix bugs
- **Documentation** - Improve or create documentation
- **Bug reports** - Report issues you encounter
- **Feature requests** - Suggest new features or improvements
- **Game content** - Contribute new items, abilities, or game world elements
- **Playtesting** - Test gameplay and provide feedback

## Pull Request Process

1. Ensure your code adheres to the project's coding standards
2. Update the `README.md` with details of changes if applicable
3. Update the Schema files if you're adding new data structures
4. Your PR should contain a clear description of the changes and their purpose
5. The PR must pass all automated tests
6. A project maintainer will review your PR and may request changes
7. Once approved, your PR will be merged

## Coding Standards

- Use consistent indentation (spaces or tabs as specified in the project)
- Follow naming conventions:
  - PascalCase for classes and methods
  - camelCase for variables and properties
  - snake_case for JSON properties
- Write clear, descriptive comments
- Keep methods focused on a single responsibility
- Write unit tests for new functionality when applicable

## Schema Guidelines

- Follow the JSON Schema [draft-07 standard](https://json-schema.org/specification-links.html#draft-07)
- Add clear descriptions for each property
- Include appropriate validation rules (type, min/max values, etc.)
- Document all changes especially breaking changes

## Issue Reporting

When reporting issues, please include:

1. A clear, descriptive title
2. A detailed description of the issue
3. Steps to reproduce the problem
4. Expected behavior
5. Actual behavior
6. Screenshots if applicable
7. Your environment

---

Thank you for contributing to Dungeon Crawler World! Your efforts help make this project better for everyone.
